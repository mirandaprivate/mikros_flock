//! Public-CSC SHA comparison: native FLOCK R1CS with C=I and full matrix checks.
#![allow(dead_code)]
#[path = "../e2e_plonk/fixture.rs"]
mod container;
#[path = "grinding.rs"]
mod grinding;

use crate::e2e_common::{TreeFixture, TreeVerifier, batched_boolean as boolean, column_boolean};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::{self, Commitment, PcsParams, functional::TensorClaim, merged_tensor, paired},
    schedule::Registry,
    zerocheck::univariate_skip::build_eq,
};
use flock_field::{F128, F256};
use grinding::GrindingChallenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env,
    io::{self, Write},
    time::Instant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Opening {
    Paired,
    Separate,
}
impl Opening {
    fn name(self) -> &'static str {
        if self == Self::Paired {
            "paired"
        } else {
            "separate"
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ReductionProof {
    Legacy(boolean::Proof),
    Column(column_boolean::Proof),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reduction {
    Legacy,
    Column,
}
impl Reduction {
    fn name(self) -> &'static str {
        match self {
            Self::Legacy => "legacy",
            Self::Column => "column",
        }
    }
}
impl ReductionProof {
    fn mode(&self) -> Reduction {
        match self {
            Self::Legacy(_) => Reduction::Legacy,
            Self::Column(_) => Reduction::Column,
        }
    }
    fn change_target(&mut self, group: usize) {
        match self {
            Self::Legacy(p) => p.target_values[group] += F128::ONE,
            Self::Column(p) => p.hadamards[group].projected_c += F128::ONE,
        }
    }
    fn swap_targets(&mut self) {
        match self {
            Self::Legacy(p) => p.target_values.swap(0, 1),
            Self::Column(p) => {
                let a = p.hadamards[0].projected_c;
                p.hadamards[0].projected_c = p.hadamards[1].projected_c;
                p.hadamards[1].projected_c = a;
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Proof {
    opening: Opening,
    commitment: Commitment,
    boolean: ReductionProof,
    nonces: Vec<u64>,
    pairing_nonce: Option<u64>,
    pcs: merged_tensor::Proof,
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn basis() -> Vec<F128> {
    (0..128)
        .map(|j| F128::new((1u128 << j) as u64, ((1u128 << j) >> 64) as u64))
        .collect()
}
fn pairs(point: &[F128]) -> Vec<[F128; 2]> {
    point.iter().map(|&r| [F128::ONE + r, r]).collect()
}
fn bind(vk: &TreeVerifier<'_>, cm: &Commitment) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-sha256-two-F128-groups-native-paired-F256-pcs-v1");
    ch.observe_bytes(&vk.circuit.digest());
    ch.observe_f128_slice(vk.public);
    ch.observe_bytes(&256u64.to_le_bytes());
    ch.observe_bytes(&(vk.r1cs.k_log as u64).to_le_bytes());
    ch.observe_bytes(&bincode::serialize(cm).unwrap());
    ch
}

/// Public source table only. No witness is available to this routine.
/// SHA schema Z word columns are IV[0,1], output[2,3], message[4,5,6,7].
fn public_io(vk: &TreeVerifier<'_>) -> Vec<F256> {
    assert_eq!(vk.live_rows, 256);
    assert_eq!(vk.public.len(), 2 + 6 * 256);
    (0..1024)
        .into_par_iter()
        .map(|c| {
            let (word, bit) = (c / 128, c % 128);
            let values: [F128; 2] = std::array::from_fn(|group| {
                let mut packed = 0u128;
                for lane in 0..128 {
                    let instance = group * 128 + lane;
                    let address = match word {
                        0 | 1 => word,
                        2 | 3 => 2 + 4 * 256 + 2 * instance + word - 2,
                        _ => 2 + 4 * instance + word - 4,
                    };
                    let value = vk.public[address];
                    let raw = u128::from(value.lo) | (u128::from(value.hi) << 64);
                    packed |= ((raw >> bit) & 1) << lane;
                }
                F128::new(packed as u64, (packed >> 64) as u64)
            });
            F256::new(values[0], values[1])
        })
        .collect()
}

fn public_claims<C: Challenger>(
    vk: &TreeVerifier<'_>,
    io: &[F256],
    ch: &mut C,
) -> Vec<paired::Claim> {
    ch.observe_label(b"sha256-all-public-io-first-1024-bits-v1");
    let mut point = ch.sample_f128_vec(10);
    let weights = build_eq(&point);
    let value = io
        .par_iter()
        .zip(&weights)
        .map(|(&v, &w)| v * w)
        .reduce(|| F256::ZERO, |a, b| a + b);
    point.resize(vk.r1cs.k_log, F128::ZERO);
    let pin = vk.r1cs.const_pin.expect("SHA constant pin");
    let pin_point: Vec<_> = (0..vk.r1cs.k_log)
        .map(|i| {
            if pin >> i & 1 == 1 {
                F128::ONE
            } else {
                F128::ZERO
            }
        })
        .collect();
    let ones = F128::new(u64::MAX, u64::MAX);
    vec![
        paired::Claim::at_point(&point, value),
        paired::Claim::at_point(&pin_point, F256::new(ones, ones)),
    ]
}

fn opening_claims<C: Challenger>(
    opening: Opening,
    claims: &[paired::Claim],
    nonce: Option<u64>,
    proving: bool,
    params: &PcsParams,
    ch: &mut C,
) -> Result<(Option<u64>, Vec<TensorClaim>), String> {
    ch.observe_label(b"sha256-opening-mode-after-F128-reduction-v1");
    ch.observe_label(opening.name().as_bytes());
    match opening {
        Opening::Paired if proving => {
            let (nonce, claims) = paired::reduce_prover(claims, params.opening_grinding(), ch)
                .map_err(|e| format!("paired reduction: {e:?}"))?;
            Ok((Some(nonce), claims))
        }
        Opening::Paired => {
            let claims = paired::reduce_verify(
                claims,
                nonce.ok_or("missing pairing nonce")?,
                params.opening_grinding(),
                ch,
            )
            .map_err(|e| format!("paired verification: {e:?}"))?;
            Ok((nonce, claims))
        }
        Opening::Separate => {
            if nonce.is_some() {
                return Err("unexpected separate-mode pairing nonce".into());
            }
            let mut out = Vec::new();
            for claim in claims {
                for group in 0..2 {
                    let mut rows = vec![if group == 0 {
                        [F128::ONE, F128::ZERO]
                    } else {
                        [F128::ZERO, F128::ONE]
                    }];
                    rows.extend_from_slice(&claim.rows);
                    out.push(TensorClaim {
                        columns: claim.columns.clone(),
                        rows,
                        value: claim.value.coordinates()[group],
                    });
                }
            }
            Ok((None, out))
        }
    }
}

fn prove(
    f: &TreeFixture,
    transposed: &boolean::TransposedWitness,
    owned_z: Vec<F128>,
    io: &[F256],
    params: &PcsParams,
    registry: &Registry,
    opening: Opening,
    reduction: Reduction,
) -> Result<(Proof, serde_json::Value, FsChallenger), String> {
    let vk = f.verifier();
    let union = container::union(registry);
    let begin = Instant::now();
    let t = Instant::now();
    let (commitment, data) = pcs::commit(&owned_z, params);
    let commit_ms = ms(t);
    let mut ch = GrindingChallenger::prover(bind(&vk, &commitment));
    let t = Instant::now();
    let (boolean, target, other) = match reduction {
        Reduction::Legacy => {
            let (proof, claims) = boolean::prove(f, transposed, &mut ch)?;
            let target = paired::Claim::at_point(
                &claims.target_point,
                F256::new(claims.target_values[0], claims.target_values[1]),
            );
            (ReductionProof::Legacy(proof), vec![target], claims.other)
        }
        Reduction::Column => {
            let (proof, claims) = column_boolean::prove(f, transposed, &mut ch)?;
            (ReductionProof::Column(proof), vec![], claims.other)
        }
    };
    let reduction_ms = ms(t);
    let t = Instant::now();
    let mut paired_claims = target;
    paired_claims.extend(public_claims(&vk, io, &mut ch));
    let io_ms = ms(t);
    let (mut ch, nonces) = ch.finish()?;
    let t = Instant::now();
    let (pairing_nonce, mut queries) =
        opening_claims(opening, &paired_claims, None, true, params, &mut ch)?;
    let pair_reduction_ms = ms(t);
    queries.extend(other);
    let source_claims = queries.len();
    let t = Instant::now();
    let pcs = merged_tensor::open_identity(owned_z, &data, &commitment, &queries, &union, &mut ch)
        .map_err(|e| format!("native PCS opening: {e:?}"))?;
    let pcs_open_ms = ms(t);
    let ring_claims = pcs.merged.as_ref().map_or(0, |p| p.ring_switches.len());
    let times = json!({"commit_ms":commit_ms,"reduction_f128_ms":reduction_ms,"public_io_ms":io_ms,
        "pair_reduction_ms":pair_reduction_ms,"pcs_open_ms":pcs_open_ms,"prover_ms":ms(begin),
        "source_claims":source_claims,"ring_claims":ring_claims,"inner_openings":1});
    Ok((
        Proof {
            opening,
            commitment,
            boolean,
            nonces,
            pairing_nonce,
            pcs,
        },
        times,
        ch,
    ))
}

fn verify(
    vk: &TreeVerifier<'_>,
    io: &[F256],
    params: &PcsParams,
    registry: &Registry,
    proof: &Proof,
    expected_opening: Opening,
    expected_reduction: Reduction,
) -> Result<FsChallenger, String> {
    if proof.opening != expected_opening
        || proof.boolean.mode() != expected_reduction
        || proof.commitment.params != *params
    {
        return Err("proof mode/PCS parameters".into());
    }
    let union = container::union(registry);
    let mut ch = GrindingChallenger::verifier(bind(vk, &proof.commitment), &proof.nonces);
    let (target, other) = match &proof.boolean {
        ReductionProof::Legacy(proof) => {
            let claims = boolean::verify(vk, proof, &mut ch)?;
            (
                vec![paired::Claim::at_point(
                    &claims.target_point,
                    F256::new(claims.target_values[0], claims.target_values[1]),
                )],
                claims.other,
            )
        }
        ReductionProof::Column(proof) => {
            let claims = column_boolean::verify(vk, proof, &mut ch)?;
            (vec![], claims.other)
        }
    };
    let mut paired_claims = target;
    paired_claims.extend(public_claims(vk, io, &mut ch));
    let (mut ch, _) = ch.finish()?;
    let (_, mut queries) = opening_claims(
        proof.opening,
        &paired_claims,
        proof.pairing_nonce,
        false,
        params,
        &mut ch,
    )?;
    queries.extend(other);
    merged_tensor::verify(&proof.commitment, &queries, &proof.pcs, &union, &mut ch)
        .map_err(|e| format!("native PCS verification: {e:?}"))?;
    Ok(ch)
}

fn tamper(
    vk: &TreeVerifier<'_>,
    io: &[F256],
    params: &PcsParams,
    registry: &Registry,
    proof: &Proof,
) -> Result<usize, String> {
    let reject = |p: &Proof| {
        if verify(
            vk,
            io,
            params,
            registry,
            p,
            proof.opening,
            proof.boolean.mode(),
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("tampered proof accepted".to_owned())
        }
    };
    let mut count = 0;
    let wrong_reduction = if proof.boolean.mode() == Reduction::Column {
        Reduction::Legacy
    } else {
        Reduction::Column
    };
    if verify(
        vk,
        io,
        params,
        registry,
        proof,
        proof.opening,
        wrong_reduction,
    )
    .is_ok()
    {
        return Err("wrong reduction mode accepted".into());
    }
    count += 1;
    let wrong_opening = if proof.opening == Opening::Paired {
        Opening::Separate
    } else {
        Opening::Paired
    };
    if verify(
        vk,
        io,
        params,
        registry,
        proof,
        wrong_opening,
        proof.boolean.mode(),
    )
    .is_ok()
    {
        return Err("wrong opening mode accepted".into());
    }
    count += 1;
    let groups = if proof.boolean.mode() == Reduction::Column {
        2
    } else {
        1
    };
    for group in 0..groups {
        for field in 0..3 {
            let mut bad = proof.clone();
            match &mut bad.boolean {
                ReductionProof::Legacy(p) => match field {
                    0 => p.hadamard.quotients[0] += F128::ONE,
                    1 => p.hadamard.inner_product.final_a += F128::ONE,
                    _ => p.hadamard.inner_product.final_b += F128::ONE,
                },
                ReductionProof::Column(p) => match field {
                    0 => p.hadamards[group].quotients[0] += F128::ONE,
                    1 => p.hadamards[group].inner_product.final_a += F128::ONE,
                    _ => p.hadamards[group].inner_product.final_b += F128::ONE,
                },
            }
            reject(&bad)?;
            count += 1;
        }
    }
    for side in 0..2 {
        let fields = if groups == 2 { 5 } else { 3 };
        for field in 0..fields {
            let mut bad = proof.clone();
            match &mut bad.boolean {
                ReductionProof::Legacy(p) => match field {
                    0 => p.linchecks[side].h_plus[7] += F128::ONE,
                    1 => p.linchecks[side].inner_product.final_a += F128::ONE,
                    _ => p.linchecks[side].inner_product.final_b += F128::ONE,
                },
                ReductionProof::Column(p) => match field {
                    0 => p.linchecks[side].columns[0][7] += F128::ONE,
                    1 => p.linchecks[side].ip.final_a += F128::ONE,
                    2 => p.linchecks[side].ip.final_b += F128::ONE,
                    3 => p.linchecks[side].ip.rounds[0][0] += F128::ONE,
                    _ => {
                        p.linchecks[side].columns.pop();
                    }
                },
            }
            reject(&bad)?;
            count += 1;
        }
    }
    if groups == 2 {
        for field in 0..4 {
            let mut bad = proof.clone();
            let ReductionProof::Column(p) = &mut bad.boolean else {
                unreachable!()
            };
            match field {
                0 => p.hadamards.swap(0, 1),
                1 => p.linchecks[0].columns.swap(0, 1),
                2 => {
                    p.hadamards.pop();
                }
                _ => {
                    p.linchecks.pop();
                }
            }
            reject(&bad)?;
            count += 1;
        }
    }
    // Keep the circuit transcript fixed: these failures must come from
    // actual public-matrix verification, not only a changed statement hash.
    for side in 0..3 {
        let mut r1cs = vk.r1cs.clone();
        let k = 1usize << r1cs.k_log;
        let zero = flock_core::r1cs::SparseBinaryMatrix::new(k, k, vec![vec![]; k]);
        match side {
            0 => r1cs.a_0 = zero,
            1 => r1cs.b_0 = zero,
            _ => r1cs.c_0 = zero,
        }
        let bad_vk = TreeVerifier { r1cs: &r1cs, ..*vk };
        if verify(
            &bad_vk,
            io,
            params,
            registry,
            proof,
            proof.opening,
            proof.boolean.mode(),
        )
        .is_ok()
        {
            return Err("changed public CSC or nonidentity C accepted".into());
        }
        count += 1;
    }
    for group in 0..2 {
        let mut bad = proof.clone();
        bad.boolean.change_target(group);
        reject(&bad)?;
        count += 1;
    }
    let mut bad = proof.clone();
    bad.boolean.swap_targets();
    reject(&bad)?;
    count += 1;
    let mut bad = proof.clone();
    bad.commitment.cap[0][0] ^= 1;
    reject(&bad)?;
    count += 1;
    let mut bad = proof.clone();
    bad.nonces.pop();
    reject(&bad)?;
    count += 1;
    let mut bad = proof.clone();
    bad.pcs.merged.as_mut().unwrap().q_eval += F128::ONE;
    reject(&bad)?;
    count += 1;
    if let Some(nonce) = proof.pairing_nonce {
        let mut bad = proof.clone();
        bad.pairing_nonce = Some(nonce ^ 1);
        reject(&bad)?;
        count += 1;
    }
    for row in [0, 127, 128, 255] {
        let mut public = vk.public.to_vec();
        public[2 + 4 * 256 + 2 * row] += F128::ONE;
        let bad_vk = TreeVerifier {
            public: &public,
            ..*vk
        };
        let bad_io = public_io(&bad_vk);
        if verify(
            &bad_vk,
            &bad_io,
            params,
            registry,
            proof,
            proof.opening,
            proof.boolean.mode(),
        )
        .is_ok()
        {
            return Err("tampered public SHA output accepted".into());
        }
        count += 1;
    }
    Ok(count)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut threads, mut reps, mut warmups, mut check) = (64, 6, 1, false);
    let mut modes = vec![Opening::Separate, Opening::Paired];
    let mut reduction = Reduction::Column;
    let mut args = env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--tamper" {
            check = true;
            continue;
        }
        let value = args.next().ok_or("missing argument")?;
        match flag.as_str() {
            "--threads" => threads = value.parse()?,
            "--reps" => reps = value.parse()?,
            "--warmups" => warmups = value.parse()?,
            "--reduction" => {
                reduction = match value.as_str() {
                    "legacy" => Reduction::Legacy,
                    "column" => Reduction::Column,
                    _ => return Err("unknown reduction".into()),
                }
            }
            "--opening" => {
                modes = match value.as_str() {
                    "paired" => vec![Opening::Paired],
                    "separate" => vec![Opening::Separate],
                    "both" => vec![Opening::Separate, Opening::Paired],
                    _ => return Err("unknown opening".into()),
                }
            }
            _ => return Err(format!("unknown argument {flag}").into()),
        }
    }
    if threads == 0 || reps == 0 || env::var_os("FLOCK_NO_GRIND").is_some() {
        return Err("invalid run configuration".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("all-core pool initialized".into());
    }
    let t = Instant::now();
    let fixture = TreeFixture::independent_compressions(256);
    let vk = fixture.verifier();
    let io = public_io(&vk);
    let prepared = fixture.prepare_witness();
    let offline_witness_ms = ms(t);
    let t = Instant::now();
    let transposed = boolean::prepare(&prepared, fixture.nu, fixture.r1cs.k_log)?;
    let offline_transpose_ms = ms(t);
    let log_words = transposed.z.len().ilog2() as usize;
    let registry = container::registry(log_words, log_words);
    let union = container::union(&registry);
    let mut params = container::parameters(&union);
    // Full rectangle: ordinary physical order and a full lane grid coincide.
    params.num_lanes = None;
    let config = params.ligerito_prover_config()?;
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(
        out,
        "{}",
        json!({"record":"metadata","schema_version":2,"reduction":reduction.name(),"fixture":"sha256-independent-compressions","sha_compressions":256,
        "threads":threads,"all_core_threads":flock_parallel::all_core_pool().current_num_threads(),"reduction_field_bits":128,
        "opening_logical_field_bits":256,"instance_groups":2,"instances_per_group":128,"pcs_carrier_field_bits":128,
        "carrier_words":transposed.z.len(),"logical_f256_words":transposed.z.len()/2,"carrier_bytes":transposed.z.len()*16,
        "pcs_params":params,"pcs_queries":config.queries,"offline_witness_ms":offline_witness_ms,"offline_transpose_ms":offline_transpose_ms,
        "fixture_digest_blake3":blake3::hash(&bincode::serialize(&(&fixture.compressions,&fixture.public))?).to_hex().to_string(),
        "circuit_digest_blake3":fixture.circuit.digest().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "r1cs_digest_blake3":fixture.r1cs.statement_digest().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "r1cs_c_identity":fixture.r1cs.c0_is_identity(),"r1cs_k_log":fixture.r1cs.k_log,"native_k_skip":fixture.r1cs.k_skip,
        "prepared_witness_blake3":blake3::hash(&bincode::serialize(&prepared)?).to_hex().to_string(),
        "matrix_authentication":"public CSC; complete verifier checks both matrix terminals; no matrix/read PCS",
        "packed_direct":env::var("FLOCK_PCS_PACKED_DIRECT").unwrap_or_default(),
        "engineering_environment":(["MIKROS_PUBLIC_CSC_SINGLE","MIKROS_PCS_LOW_BIT_SUPPORT","MIKROS_PCS_REUSE_UNIT_TABLE",
          "MIKROS_HADAMARD_WIDE_PACK","MIKROS_HADAMARD_GROUP_TILES","MIKROS_HADAMARD_TILE_ROWS","MIKROS_IP_BATCH_PARALLEL",
          "MIKROS_IMAGE_BLOCKED_WEIGHTS","MIKROS_SERIAL_DEINTERLEAVE","MIKROS_HADAMARD_GFNI_QUOTIENT"]
          .into_iter().map(|key| (key,env::var(key).ok())).collect::<std::collections::BTreeMap<_,_>>()),
        "target_features":features(),"zero_knowledge":false,
        "timing":"offline witness/transposition and input clone excluded; online F128 reduction, IO checks, pairing, commitment, native PCS and cleanup included",
        "relation":"A0 Z bitwise-product B0 Z = Z; SHA C0=I. Two groups of128instancebits; public IO and pin PCS-authenticated.",
        "linear_reduction":if reduction == Reduction::Column { "separate Hadamards with shared row challenges and independent column MLE queries; fix all block column messages, fresh block weights and128coordinate projection, one ordinary IP per A/B on fixed-basis witness; public CSC endpoints checked directly" } else { "legacy weighted residual batch and packed extension IP with query-dependent bit weights" },
        "comparison":"same SHA compression inputs/outputs and native PCS backend; Mikros transposed full carrier vs native FLOCK compacted witness, layouts and commitment hashes differ"})
    )?;
    out.flush()?;
    let mut hashes = std::collections::BTreeMap::new();
    let mut common_piop = None;
    for iteration in 0..warmups + reps {
        let mut order = modes.clone();
        if iteration % 2 == 1 {
            order.reverse();
        }
        for opening in order {
            let owned = transposed.z.clone();
            let t = Instant::now();
            let (proof, mut timings, mut ch) = prove(
                &fixture,
                &transposed,
                owned,
                &io,
                &params,
                &registry,
                opening,
                reduction,
            )?;
            timings["prover_ms"] = json!(ms(t));
            let t = Instant::now();
            let mut vch = verify(&vk, &io, &params, &registry, &proof, opening, reduction)?;
            let verify_ms = ms(t);
            if ch.sample_f128() != vch.sample_f128() {
                return Err("transcript mismatch".into());
            }
            let encoded = bincode::serialize(&proof)?;
            let hash = blake3::hash(&encoded).to_hex().to_string();
            if hashes
                .insert(opening.name(), hash.clone())
                .is_some_and(|x| x != hash)
            {
                return Err("unstable proof bytes".into());
            }
            let piop = blake3::hash(&bincode::serialize(&(&proof.boolean, &proof.nonces))?)
                .to_hex()
                .to_string();
            if common_piop.as_ref().is_some_and(|x| x != &piop) {
                return Err("pairing changed F128 reduction".into());
            }
            common_piop = Some(piop.clone());
            writeln!(
                out,
                "{}",
                json!({"record":"sample","system":"mikros","reduction":reduction.name(),"opening":opening.name(),"iteration":iteration,"warmup":iteration<warmups,
                "timings":timings,"verify_ms":verify_ms,"proof_bytes":encoded.len(),"proof_blake3":hash,"piop_blake3":piop,
                "commitment_blake3":blake3::hash(&bincode::serialize(&proof.commitment)?).to_hex().to_string(),"verified":true,"transcript_match":true})
            )?;
            out.flush()?;
            if check && iteration == 0 {
                let rejected = tamper(&vk, &io, &params, &registry, &proof)?;
                writeln!(
                    out,
                    "{}",
                    json!({"record":"negative_tests","opening":opening.name(),"rejected":rejected})
                )?;
                out.flush()?;
            }
        }
    }
    Ok(())
}

fn features() -> Vec<&'static str> {
    [
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ]
    .into_iter()
    .filter_map(|(n, on)| on.then_some(n))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_256_public_sha_inputs_outputs_and_pin_match_committed_layout() {
        let f = TreeFixture::independent_compressions(256);
        let vk = f.verifier();
        let io = public_io(&vk);
        let t = boolean::prepare(&f.prepare_witness(), f.nu, f.r1cs.k_log).unwrap();
        for (constraint, expected) in io.iter().enumerate() {
            assert_eq!(
                *expected,
                F256::new(t.z[2 * constraint], t.z[2 * constraint + 1])
            );
        }
        let pin = f.r1cs.const_pin.unwrap();
        assert_eq!(t.z[2 * pin], F128::new(u64::MAX, u64::MAX));
        assert_eq!(t.z[2 * pin + 1], F128::new(u64::MAX, u64::MAX));
        // Each endpoint group has an independent public output obligation.
        for instance in [0, 127, 128, 255] {
            let mut public = f.public.clone();
            public[2 + 4 * 256 + 2 * instance] += F128::ONE;
            let bad = TreeVerifier {
                public: &public,
                ..vk
            };
            assert_ne!(public_io(&bad), io);
        }
    }
}
