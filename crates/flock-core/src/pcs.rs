//! Polynomial commitment scheme for the bit-MLE witness `ẑ` over GF(2).
//!
//! Construction: Binius-style PCS with F_{2^128} packing.
//!
//! - **Commit**: pack the 2^m Boolean witness into 2^(m−7) F_{2^128} elements
//!   (one bit per polynomial-basis coordinate of F_{2^128}), batch RS-encode
//!   via additive NTT, Merkle-commit the codeword.
//! - **Open**: at a QuirkyPoint (z_skip, x_outer) from the zerocheck/lincheck:
//!   1. [`ring_switch::prove`] sends 128 partial-evaluations `s_hat_v` and
//!      produces a sumcheck target `(rs_eq_ind, sumcheck_claim)`.
//!   2. [`ligerito::recursive_prover_with_basis`] discharges the combined
//!      claim `⟨packed_witness, b_combined⟩ = target_combined` via the
//!      recursive Ligerito argument, reusing the commit-time codeword and
//!      Merkle tree as Ligerito's L0 commitment.
//! - **Verify**: the verifier replays ring-switching succinctly, then drives
//!   the succinct recursive Ligerito verifier, evaluating the combined basis
//!   at the residual point (see [`verify_opening_batch_ligerito_mixed`]).
//!
//! See [DP24](https://eprint.iacr.org/2024/504) (ring-switching) and the
//! ligerito module docs for the recursion.

use std::{env::var, sync::atomic::AtomicU8, time::Instant};

pub use commit::{
    Commitment, PcsParams, ProverData, commit, commit_encode, commit_encode_into, commit_into,
    commit_lane_major, commit_leaf_pipeline_shape, commit_merkle, dense_lanes,
    prefault_codeword_during,
};
pub use jagged::rectangular_prefix_columns;
use jagged::{
    FrobeniusClaim, JaggedParams, MergedWeightClaim, MultipointDefer, MultipointGrinding,
    MultipointTwistedProof, ScalarGroupClaim, fold_round_claim,
    prove_multipoint_twisted_with_grinding, verify_multipoint_twisted_deferred_with_grinding,
    verify_multipoint_twisted_with_grinding,
};
use ligerito::{
    BasisWindowFn, FoldLookahead, LigeritoProof, ProverConfig, VerifierConfig, VirtualEqBasis,
    extension::recursive_verifier_with_basis_succinct, lookahead_accum_group, lookahead_finish,
    recursive_prover_with_basis_precomputed_round0_lanes, xor_acc8,
};
pub use pack::{LOG_PACKING, pack_witness};
use rayon::{
    current_num_threads, join,
    prelude::{
        IndexedParallelIterator, IntoParallelIterator, ParallelIterator, ParallelSlice,
        ParallelSliceMut,
    },
};
use ring_switch::{
    RingSwitchError, RsEqInd, build_eq_scaled_parallel, build_fold_byte_table,
    eval_rs_eq_finish_from_prefix_binary_q_f256, eval_rs_eq_prefix_f256, fold_b128_from_table,
    fold_one_slot, linearized_coefficients,
    prove_batched_padded_with_precomputed_unbatched_and_grinding,
    verify_succinct_weighted_with_grinding, verify_succinct_with_grinding,
};
pub use ring_switch::{RingSwitchProof, SparseEqTensor};
use serde::{Deserialize, Serialize};
#[cfg(feature = "mul-count")]
use {crate::field::gf2_128::op_count::MULS_PER_INV, crate::field::gf2_128::op_count::snapshot};

use crate::{
    all_core_pool,
    challenger::Challenger,
    ecore_rich_topology,
    field::{F128, F256, F256Unreduced},
    lincheck::{SkipPoint, build_eq_table},
    matrix_fold::{JaggedAssertion, JaggedClaim, JaggedRowWeight, JaggedTable},
    merkle::cap_layer,
    pcs::tensor_algebra::TensorAlgebra256,
    scratch::{give_f128, take_f128},
    zerocheck::PaddingSpec,
};
pub mod commit;
pub mod functional;
pub mod jagged;
pub mod ligerito;
pub mod merged_tensor;
pub mod pack;
pub mod paired;
pub mod ring_switch;
pub mod stratified;
pub mod tensor_algebra;

/// Batched opening proof: ring-switching frontend + Ligerito backend.
/// The combined `b_combined` + target_combined feed
/// [`ligerito::recursive_prover_with_basis`] (see ligerito module docs).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchOpeningProofLigerito {
    pub ring_switches: Vec<RingSwitchProof>,
    /// Fiat--Shamir PoW witnesses for the random-linear-combination
    /// coefficients, in claim order (ring-switched claims first, then packed
    /// direct claims). Empty when opening grinding is disabled or when the
    /// batch has only one claim.
    #[serde(default)]
    pub batching_nonces: Vec<u64>,
    pub ligerito: LigeritoProof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PcsError {
    RingSwitch(RingSwitchError),
    /// The Ligerito recursive verifier rejected the proof.
    Ligerito,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PcsOpenError {
    RingSwitch(RingSwitchError),
    /// The virtual-opening sumcheck rejected (wrong round count, or the final
    /// round does not match `b̂_combined(ρ) · f_eval`).
    VirtualOpen,
    /// The assist layer rejected (the Frobenius-assist replay failed), or a
    /// claim-shape mismatch reached the merged open.
    Assist,
    /// The Ligerito recursive verifier rejected the opening (also raised by
    /// the merged verifier's commitment-params equality check).
    Ligerito,
}

/// `eq_ind` representation for a packed-direct claim. The contributed value at
/// scattered index `j` is the tensor entry — for the dense variant the index
/// is the array offset; for the sparse variant it's reconstructed via
/// [`SparseEqTensor::scatter_idx`].
#[derive(Clone, Debug)]
pub enum DirectEqInd {
    /// Fully-materialized `eq_ind(point)` of length `2^L`.
    Dense(Vec<F128>),
    /// Deferred eq tensor: only the point rides in. Two roles:
    ///
    /// - The merged transport's inner open (a single such claim, no RS
    ///   claims): the combine materializes `γ·eq(point)` directly as
    ///   `b_combined` with a seeded build (no separate eq buffer, no
    ///   re-scale pass).
    /// - The inert carrier for OUTER merged claims and the
    ///   pre-materialization form on the union prover's jagged arm:
    ///   [`open_batch_merged`] derives its weights from `point`/`value`
    ///   alone and never reads `eq_ind`, so deferred claims ride through it
    ///   unbuilt (a claim that WOULD need a materialized tensor trips the
    ///   "EqPoint claims are only supported alone" assert in the combine
    ///   rather than dropping the contribution).
    ///
    /// Transcript-identical to `Dense` of the same point in every role —
    /// the representation is prover-side only.
    EqPoint(Vec<F128>),
    /// Sparse representation — non-zero entries at scattered indices.
    /// Built from a claim point with one or more exactly-zero coords via
    /// [`ring_switch::build_eq_sparse`].
    Sparse(SparseEqTensor),
}

/// A packed-MLE evaluation claim: `ẑ_packed(point) = value`. Unlike a
/// ring-switched claim, this is opened directly without going through the
/// bit-MLE ↔ packed-MLE bridge (no `s_hat_v`, no φ_8 weighting).
///
/// Use case: protocols whose sumcheck output is naturally a packed-MLE
/// evaluation (e.g. the chain shift sumcheck operating on packed columns
/// instead of bit-folded scalars). Skips the ring-switch step for this claim,
/// saving the `fold_1b_rows` + per-opening-tail work at the prover and the
/// ring-switch verify + φ_8 reconstruction at the verifier.
///
/// The claim-combine step adds `γ_k · eq_ind(point)` to `b_combined` and
/// `γ_k · value` to the target; the verifier's residual check contributes
/// `γ_k · eq_eval(point, residual_challenges)`.
#[derive(Clone, Debug)]
pub struct PackedDirectClaim {
    /// Multilinear point of length `L = m − 7`.
    pub point: Vec<F128>,
    /// Claimed `ẑ_packed(point)` value.
    pub value: F128,
    /// `eq_ind(point)` in dense or sparse form. Caller responsibility to
    /// match the claim's `point` — the contribution to `b_combined` is read
    /// directly from this tensor.
    pub eq_ind: DirectEqInd,
}

/// In-process A/B override for the VIRTUAL BASIS (see
/// [`ligerito::VirtualEqBasis`]): `0` follows the `FLOCK_NO_VIRTUAL_B` env
/// knob, `1` forces the virtual basis on, `2` forces the pre-virtualization
/// path (round-0-only JIT fill). A few-ms effect only resolves under an
/// ALTERNATING in-process instrument over identical inputs — process-level
/// arms on this box carry ±4-8 ms of interference per sample.
pub static VIRTUAL_B_OVERRIDE: AtomicU8 = AtomicU8::new(0);

/// In-process A/B override for the STATISTICS LADDER (see
/// `ligerito::extension::init_phase_statistics`): `0` follows the
/// `FLOCK_NO_STATS_LADDER` env knob, `1` forces it on, `2` forces the
/// incremental L0 fold ladder. Both arms produce byte-identical proofs.
pub static STATS_LADDER_OVERRIDE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

pub(crate) fn virtual_b_enabled() -> bool {
    match VIRTUAL_B_OVERRIDE.load(std::sync::atomic::Ordering::Relaxed) {
        1 => true,
        2 => false,
        _ => std::env::var_os("FLOCK_NO_VIRTUAL_B").is_none(),
    }
}

pub(crate) fn stats_ladder_enabled() -> bool {
    match STATS_LADDER_OVERRIDE.load(std::sync::atomic::Ordering::Relaxed) {
        1 => true,
        2 => false,
        _ => std::env::var_os("FLOCK_NO_STATS_LADDER").is_none(),
    }
}

/// Fiat--Shamir grinding policy for the PCS transport that sits before the
/// Ligerito opening.  Each nonzero field is applied immediately after its
/// prover message(s) are bound and immediately before the challenge it
/// protects is sampled.
///
/// The Secure schedule turns a degree-/numerator-`D` algebraic event over
/// `F128` into a computational work-factor term of about
/// `D * 2^-bits / 2^128` at that site:
///
/// * ring switching has degree at most seven;
/// * each random linear-combination coefficient has numerator one;
/// * the merged, multipoint, and anchor sumchecks have quadratic rounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpeningGrinding {
    /// Seven-coordinate ring-switch point `r''`.
    pub ring_switch_bits: u32,
    /// Random coefficients used to batch independent opening claims.
    pub claim_batch_bits: u32,
    /// Each quadratic round of the dense merged sumcheck.
    pub merged_round_bits: u32,
    /// The multipoint dual-value batching coefficient and its sumcheck /
    /// anchor-round policies.
    pub multipoint: MultipointGrinding,
}

impl OpeningGrinding {
    pub const fn disabled() -> Self {
        Self {
            ring_switch_bits: 0,
            claim_batch_bits: 0,
            merged_round_bits: 0,
            multipoint: MultipointGrinding::disabled(),
        }
    }

    pub const fn per_challenge_128() -> Self {
        Self {
            // 7 / 2^128, then a 2^-3 PoW filter.
            ring_switch_bits: 3,
            // The transport coefficient is sampled before Ligerito's OOD
            // points bind one candidate from the Johnson list. Its bad event
            // therefore unions over L0's list (log2 L < 6 in every embedded
            // strict profile); six PoW bits make L/2^(128+6) < 2^-128.
            claim_batch_bits: 6,
            // Quadratic sumcheck rounds: 2 / 2^128, then 2^-2.
            merged_round_bits: 2,
            multipoint: MultipointGrinding::per_challenge_128(),
        }
    }

    #[inline]
    fn claim_batch_bits_for(self, claim_count: usize) -> u32 {
        if claim_count > 1 {
            self.claim_batch_bits
        } else {
            0
        }
    }
}

/// Mixed-claim batched open: supports both **ring-switched** claims (bit-MLE
/// openings reduced via `ring_switch::prove_batched`, with optional per-claim
/// precomputed `s_hat_v`) and **packed-direct** claims (packed-MLE openings
/// that skip ring-switch). Runs the ring_switch + b_combined computation, then
/// routes to [`ligerito::recursive_prover_with_basis`] using the existing
/// `prover_data`'s codeword + tree as Ligerito's L0 commit (no L0 re-commit),
/// with PoW witnesses for ring switching and nontrivial claim batching
/// (pass [`OpeningGrinding::disabled`] for a grind-free transcript).
///
/// `lig_config.initial_k` must equal `commitment.params.log_batch_size` so that
/// `prover_data`'s codeword/tree shape matches what Ligerito expects for L0.
#[allow(clippy::too_many_arguments)]
pub fn open_batch_mixed_ligerito_with_precomputed_s_hat_v_and_grinding<Ch: Challenger>(
    packed_witness: Vec<F128>,
    prover_data: &ProverData,
    commitment: &Commitment,
    x_outers: &[&[F128]],
    precomputed_s_hat_v: &[Option<&[F128]>],
    packed_direct: &[PackedDirectClaim],
    padding: &PaddingSpec,
    lig_config: &ProverConfig,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
) -> BatchOpeningProofLigerito {
    open_batch_mixed_ligerito_seeded(
        packed_witness,
        prover_data,
        commitment,
        x_outers,
        precomputed_s_hat_v,
        packed_direct,
        padding,
        lig_config,
        grinding,
        None,
        challenger,
    )
}

/// [`open_batch_mixed_ligerito_with_precomputed_s_hat_v_and_grinding`] with
/// the inner open's SEEDED BLOCK STATISTICS supplied by the caller: the
/// merged transport's inner claim sits at the merged sumcheck's own point,
/// so that sumcheck's folded witness after `log_n − initial_k` rounds IS
/// `Σ_h f[e,h]·eq(ρ_{low}, h)` per lane block — the statistics ladder's
/// seeded term, which it would otherwise sweep the witness to recompute.
#[allow(clippy::too_many_arguments)]
pub fn open_batch_mixed_ligerito_seeded<Ch: Challenger>(
    packed_witness: Vec<F128>,
    prover_data: &ProverData,
    commitment: &Commitment,
    x_outers: &[&[F128]],
    precomputed_s_hat_v: &[Option<&[F128]>],
    packed_direct: &[PackedDirectClaim],
    padding: &PaddingSpec,
    lig_config: &ligerito::ProverConfig,
    grinding: OpeningGrinding,
    seeded_stats: Option<Vec<F128>>,
    challenger: &mut Ch,
) -> BatchOpeningProofLigerito {
    // Belt-and-braces on the cap-depth derivation: the commit-time cap
    // (from `PcsParams::level_zero_cap_depth`) must be the layer the opener's
    // config implies — a config-source disagreement fails loudly here at
    // prove time instead of as a verifier reject.
    assert_eq!(
        commitment.cap.len(),
        1usize << lig_config.level_zero_cap_depth(),
        "commitment cap size disagrees with the opener config's L0 query count"
    );
    debug_assert_eq!(
        commitment.cap.as_slice(),
        cap_layer(
            &prover_data.merkle_tree,
            commitment.params.n_leaves(),
            lig_config.level_zero_cap_depth(),
        ),
        "commitment cap is not the prover tree's cap layer"
    );
    let trace = var("PCS_TRACE").is_ok();
    let t_total = Instant::now();

    assert_eq!(
        lig_config.initial_k, commitment.params.log_batch_size,
        "ligerito initial_k ({}) must match PcsParams.log_batch_size ({}) for L0 reuse",
        lig_config.initial_k, commitment.params.log_batch_size,
    );
    assert_eq!(
        lig_config.log_inv_rates[0], commitment.params.log_inv_rate,
        "ligerito log_inv_rates[0] ({}) must match PcsParams.log_inv_rate ({}) for L0 reuse",
        lig_config.log_inv_rates[0], commitment.params.log_inv_rate,
    );

    // Integer-lane (lane-major) commitments are supported ONLY in the
    // merged transport's inner-open configuration (no RS claims, one
    // EqPoint claim): the eq basis folds honestly over the zero-padding
    // lanes (live-block skip disabled) and the round-0 prime pairs blocks.
    let l0_num_lanes = commitment.params.num_ntts();
    let lane_major = l0_num_lanes < 1usize << lig_config.initial_k;
    let log_n = commitment.params.m - LOG_PACKING;
    if lane_major {
        assert!(
            x_outers.is_empty()
                && packed_direct.len() == 1
                && matches!(packed_direct[0].eq_ind, DirectEqInd::EqPoint(_)),
            "lane-major mixed open: only the merged inner-open configuration is supported"
        );
    }
    let round0_block = if lane_major {
        1usize << (log_n - lig_config.initial_k)
    } else {
        1
    };
    let combined = compute_combined_basis_and_target(
        &packed_witness,
        x_outers,
        precomputed_s_hat_v,
        packed_direct,
        padding,
        round0_block,
        grinding,
        challenger,
        trace,
    );

    let t = Instant::now();
    let CombinedClaim {
        ring_switches,
        batching_nonces,
        b_combined,
        eq_basis,
        eq_gamma,
        target_combined,
        round0_prime,
        round1_lookahead,
    } = combined;
    // Factored EqPoint basis. DEFAULT (virtual): the basis stays factored
    // across EVERY L0 fold — an eq tensor folds to an eq tensor, so no round
    // writes or reads a half-size basis array; it materializes once, at the
    // last L0 round, at the size the recursion takes over
    // ([`ligerito::VirtualEqBasis`]). `FLOCK_NO_VIRTUAL_B=1` falls back to
    // the round-0-only JIT fill (the pre-virtualization behavior) — the
    // CERTIFICATION knob for alternating A/B runs; both settings produce the
    // same b values, hence byte-identical proofs.
    // Lane-major only: the virtual basis rides the BLOCKED L0 fold, which is
    // what every shipped merged-transport open runs (a pow2-lane inner keeps
    // the tuned element-pairing kernel and its round-0 JIT).
    let virtual_b = eq_basis.is_some() && lane_major && virtual_b_enabled();
    let vbasis = if virtual_b {
        // The point IS the claim's, and γ is its single transcript scalar —
        // the same (γ, ρ) the split tables above were seeded with.
        Some(VirtualEqBasis::new(
            match &packed_direct[0].eq_ind {
                DirectEqInd::EqPoint(point) => point.clone(),
                _ => unreachable!("the factored basis is built only for EqPoint"),
            },
            eq_gamma.expect("an eq basis carries its γ"),
        ))
    } else {
        None
    };
    let jit_fill;
    let jit: Option<BasisWindowFn<'_>> = match (&eq_basis, virtual_b) {
        (Some((lo, hi, n_lo)), false) => {
            let mask = (1usize << n_lo) - 1;
            let n_lo = *n_lo;
            jit_fill = move |out: &mut [F128], g0: usize| {
                for (i, slot) in out.iter_mut().enumerate() {
                    let u = g0 + i;
                    *slot = lo[u & mask] * hi[u >> n_lo];
                }
            };
            Some(&jit_fill)
        }
        _ => None,
    };
    let ligerito_proof = recursive_prover_with_basis_precomputed_round0_lanes(
        lig_config,
        packed_witness,
        b_combined,
        target_combined,
        &prover_data.codeword,
        &prover_data.merkle_tree,
        l0_num_lanes,
        lane_major,
        round0_prime,
        round1_lookahead,
        jit,
        vbasis,
        seeded_stats,
        challenger,
    );
    if trace {
        eprintln!(
            "  [open_batch] ligerito::recursive_prover_with_basis: {:6.2} ms",
            t.elapsed().as_secs_f64() * 1e3
        );
        eprintln!(
            "  [open_batch] TOTAL: {:6.2} ms",
            t_total.elapsed().as_secs_f64() * 1e3
        );
    }

    BatchOpeningProofLigerito {
        ring_switches,
        batching_nonces,
        ligerito: ligerito_proof,
    }
}

/// What ring_switch + claim-combination produces, fed to the Ligerito backend.
struct CombinedClaim {
    ring_switches: Vec<RingSwitchProof>,
    /// PoW witnesses for random linear-combination coefficients.
    batching_nonces: Vec<u64>,
    /// The materialized γ-combined basis — EMPTY when `eq_basis` is `Some`
    /// (the factored path never writes the full-domain array).
    b_combined: Vec<F128>,
    /// The single-EqPoint fast path's FACTORED basis: `(γ-scaled eq_lo,
    /// eq_hi, n_lo)` with `b[u] = lo[u & mask]·hi[u >> n_lo]`. γ is one
    /// fixed transcript scalar (exactly one packed-direct claim), so the
    /// whole basis is a scaled eq tensor — two √L tables instead of a
    /// 2^(m−7)-word array. The L0 first fold sources windows from it
    /// just-in-time ([`ligerito::BasisWindowFn`]); later folds carry the
    /// half-size folded basis as before.
    eq_basis: Option<(Vec<F128>, Vec<F128>, usize)>,
    /// The single γ behind `eq_basis` — the virtual basis re-seeds its own
    /// tables from `(γ, point)` as it folds.
    eq_gamma: Option<F128>,
    target_combined: F128,
    /// Round-0 sumcheck `(u_0, u_2)` prime over `packed_witness · b_combined`,
    /// consumed by `recursive_prover_with_basis_precomputed_round0`.
    round0_prime: (F128, F128),
    /// Quadratic coefficients of Ligerito's ROUND-1 message in the round-0
    /// fold challenge, accumulated in the same combine pass. Three paths
    /// emit them: the plain fast path (LSB-pair coefficients), the fast
    /// path WITH sparse packed-direct claims (scatter deltas corrected by
    /// linearity), and the seeded EqPoint path (BLOCKED coefficients, the
    /// fold's own block pairing). Lets the recursive prover's first lane
    /// fold be an O(1) skip round — see [`ligerito::FoldLookahead`].
    round1_lookahead: Option<FoldLookahead>,
}

/// Runs ring_switch over RS claims, observes packed-direct claim values +
/// samples their gammas, then builds `b_combined` (the γ-weighted linear
/// combination of all `rs_eq_ind`s and `eq_ind`s) and `target_combined`.
/// Also computes the round-0 prime as a side effect (cheap since it shares
/// the b_combined pass).
///
/// (The M6 support-proportional `live_pairs`/`stream_b` machinery lived
/// here for the jagged transport's virtual-opening sumcheck and was removed
/// with it; the dead-block skip below is the surviving support-awareness.)
#[allow(clippy::too_many_arguments)]
fn compute_combined_basis_and_target<Ch: Challenger>(
    packed_witness: &[F128],
    x_outers: &[&[F128]],
    precomputed_s_hat_v: &[Option<&[F128]>],
    packed_direct: &[PackedDirectClaim],
    padding: &PaddingSpec,
    // Round-0 pairing block for the EqPoint special path: 1 = adjacent
    // elements (pow2 lanes); 2^(log_n − initial_k) under a lane-major
    // commitment, whose L0 fold pairs BLOCKS. Other paths must pass 1
    // (their primes are adjacent-paired; the jagged path re-derives its
    // blocked prime downstream).
    eqpoint_round0_block: usize,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
    trace: bool,
) -> CombinedClaim {
    let n_rs = x_outers.len();
    let n_pd = packed_direct.len();
    assert!(n_rs + n_pd > 0, "open_batch_mixed: need at least one claim");
    assert!(
        precomputed_s_hat_v.is_empty() || precomputed_s_hat_v.len() == n_rs,
        "precomputed_s_hat_v: must be empty or length {n_rs}, got {}",
        precomputed_s_hat_v.len(),
    );

    challenger.observe_label(b"flock-pcs-open-batch-v0");

    // 1. Ring-switching for all x_outers.
    let t = Instant::now();
    let batch_bits = grinding.claim_batch_bits_for(n_rs + n_pd);
    let mut rs_results = if n_rs > 0 {
        prove_batched_padded_with_precomputed_unbatched_and_grinding(
            packed_witness,
            x_outers,
            precomputed_s_hat_v,
            padding,
            grinding.ring_switch_bits,
            challenger,
        )
    } else {
        Vec::new()
    };
    if trace {
        eprintln!(
            "  [open_batch] ring_switch::prove_batched ×{}: {:6.2} ms",
            n_rs,
            t.elapsed().as_secs_f64() * 1e3
        );
    }

    // 2. Observe packed-direct claim values + sample γ_pd.
    for pd in packed_direct {
        challenger.observe_label(b"flock-pcs-packed-direct-v0");
        challenger.observe_f128(pd.value);
    }
    let mut batching_nonces = Vec::with_capacity(usize::from(batch_bits != 0));
    let gammas = if batch_bits != 0 {
        let (nonce, gammas) = challenger.grind_pow_and_sample_f128_vec(batch_bits, n_rs + n_pd);
        batching_nonces.push(nonce);
        gammas
    } else {
        challenger.sample_f128_vec(n_rs + n_pd)
    };
    let (gammas_rs, gammas_pd) = gammas.split_at(n_rs);
    for ((_, output), &gamma) in rs_results.iter_mut().zip(gammas_rs) {
        output.rs_eq_ind.scale_in_place(gamma);
    }

    let t = Instant::now();

    let l = if let Some((_, out)) = rs_results.first() {
        out.rs_eq_ind.len()
    } else {
        1usize << packed_direct[0].point.len()
    };
    debug_assert!(rs_results.iter().all(|(_, o)| o.rs_eq_ind.len() == l));
    // Assemble each independent binary suffix in parallel.
    debug_assert!(
        packed_direct.iter().all(|pd| 1usize << pd.point.len() == l),
        "all packed-direct claims must share L (= packed witness length)"
    );

    let mut target_combined = F128::ZERO;
    for ((_, output), g) in rs_results.iter().zip(gammas_rs.iter()) {
        target_combined += *g * output.sumcheck_claim;
    }
    for (pd, g) in packed_direct.iter().zip(gammas_pd.iter()) {
        target_combined += *g * pd.value;
    }

    let rs_baked: Vec<&[F128]> = rs_results
        .iter()
        .filter_map(|(_, o)| match &o.rs_eq_ind {
            RsEqInd::Dense(v) => Some(v.as_slice()),
            _ => None,
        })
        .collect();
    // Deferred-dense claims (fused fast path): the per-claim `γ_k·B_k` buffer
    // was never materialized — fold each slot on the fly below and accumulate
    // straight into `b_combined`, saving a 2^(m-7) materialize + readback per
    // claim. Carries (eq_lo, eq_hi, γ-baked table, log₂ B).
    let rs_deferred: Vec<(&[F128], &[F128], &[F128], usize)> = rs_results
        .iter()
        .filter_map(|(_, o)| match &o.rs_eq_ind {
            RsEqInd::DeferredDense {
                eq_lo,
                eq_hi,
                table,
            } => Some((
                eq_lo.as_slice(),
                eq_hi.as_slice(),
                table.as_slice(),
                eq_lo.len().trailing_zeros() as usize,
            )),
            _ => None,
        })
        .collect();
    let pd_dense: Vec<(&[F128], F128)> = packed_direct
        .iter()
        .zip(gammas_pd.iter())
        .filter_map(|(pd, g)| match &pd.eq_ind {
            DirectEqInd::Dense(v) => Some((v.as_slice(), *g)),
            _ => None,
        })
        .collect();

    // Merged-transport inner open: no RS claims, exactly one EqPoint claim.
    // Build `b_combined = γ·eq(point)` seeded (one pass, no eq buffer) and
    // fuse the round-0 prime. Transcript-identical to the Dense variant of
    // the same point (the eq_ind representation is prover-side only).
    if rs_results.is_empty()
        && packed_direct.len() == 1
        && let DirectEqInd::EqPoint(point) = &packed_direct[0].eq_ind
    {
        assert_eq!(1usize << point.len(), l, "EqPoint length mismatch");
        // FACTORED basis: b = γ·eq(point,·) is one fixed γ times an eq
        // tensor, so two √L tables replace the 2^(m−7)-word array — γ baked
        // into the lo half. Exact: field ops are exact, so the split
        // product is bitwise the materialized entry.
        let n_lo = point.len() / 2;
        let lo = build_eq_scaled_parallel(&point[..n_lo], gammas_pd[0]);
        let hi = build_eq_scaled_parallel(&point[n_lo..], F128::ONE);
        let mask = (1usize << n_lo) - 1;
        let bs = |u: usize| lo[u & mask] * hi[u >> n_lo];
        let blk = eqpoint_round0_block;
        // STATISTICS LADDER: the lane-major inner open derives its round-0
        // message and every later L0 round from block statistics inside the
        // recursive prover (one sweep there covers this term AND the L0 OOD
        // term), so the O(L) prime + lookahead sweep below is skipped. Same
        // gate as the prover's: `virtual_b` = eq basis + lane-major, which
        // is exactly this branch with `blk > 1`.
        if blk > 1 && virtual_b_enabled() && stats_ladder_enabled() {
            if trace {
                eprintln!("  [open_batch] combine (seeded EqPoint, statistics ladder): no sweep");
            }
            return CombinedClaim {
                ring_switches: Vec::new(),
                batching_nonces,
                b_combined: Vec::new(),
                eq_basis: Some((lo, hi, n_lo)),
                eq_gamma: Some(gammas_pd[0]),
                target_combined,
                round0_prime: (F128::ZERO, F128::ZERO),
                round1_lookahead: None,
            };
        }
        if blk == 1 {
            // The ladder statically rejects a lookahead on this shape:
            // `fold_block == 1` comes with the factored (JIT) basis, so
            // `kind_ok` in extension.rs never consumes it. Run the lean
            // prime-only pass (2 muls/pair) instead of paying the doubled
            // lookahead accumulation for coefficients nothing reads.
            const C: usize = 1 << 13;
            let (round0_u0, round0_u2) = packed_witness
                .par_chunks(C)
                .enumerate()
                .map(|(ci, qc)| {
                    let base = ci * C;
                    let mut a = F128::ZERO;
                    let mut b = F128::ZERO;
                    for (j, qp) in qc.as_chunks::<2>().0.iter().enumerate() {
                        let u = base + 2 * j;
                        let (w0, w1) = (bs(u), bs(u + 1));
                        a += qp[0] * w0;
                        b += (qp[0] + qp[1]) * (w0 + w1);
                    }
                    (a, b)
                })
                .reduce(
                    || (F128::ZERO, F128::ZERO),
                    |(x0, x2), (y0, y2)| (x0 + y0, x2 + y2),
                );
            if trace {
                eprintln!(
                    "  [open_batch] combine (seeded EqPoint, L={l}): {:6.2} ms",
                    t.elapsed().as_secs_f64() * 1e3
                );
            }
            return CombinedClaim {
                ring_switches: Vec::new(),
                batching_nonces,
                b_combined: Vec::new(),
                eq_basis: Some((lo, hi, n_lo)),
                eq_gamma: Some(gammas_pd[0]),
                target_combined,
                round0_prime: (round0_u0, round0_u2),
                round1_lookahead: None,
            };
        }
        // The same seeded pass now also accumulates the ROUND-1 message's
        // quadratic coefficients in the round-0 fold challenge, under the
        // fold's own pairing: quad `q` covers the four consecutive
        // `blk`-blocks the ladder's first two folds combine, so
        // `ligerito::lookahead_accum_group` applies verbatim with blocked
        // gathering (+4 unreduced muls per quad on a pass that already runs;
        // it buys the ladder a full fold-0 pass). Needs `l ≥ 4·blk`
        // (initial_k ≥ 2), which every shipped ladder satisfies.
        assert!(blk.is_power_of_two() && l.is_multiple_of(4 * blk));
        let quad = |q: usize, acc: &mut [F256Unreduced; 8]| {
            let i0 = 4 * q * blk;
            for k in 0..blk {
                let i = i0 + k;
                let fq = [
                    packed_witness[i],
                    packed_witness[i + blk],
                    packed_witness[i + 2 * blk],
                    packed_witness[i + 3 * blk],
                ];
                let bq = [bs(i), bs(i + blk), bs(i + 2 * blk), bs(i + 3 * blk)];
                lookahead_accum_group(&fq, &bq, acc);
            }
        };
        // Parallelize on quads; for the lane-major shape (huge blk, few
        // quads) split each quad's inner `k` range instead.
        let n_quads = l / (4 * blk);
        let acc = if n_quads >= 64 {
            // ~2^13 elements per task, whatever the block size.
            let qc = ((1usize << 13) / (4 * blk)).max(1);
            (0..n_quads.div_ceil(qc))
                .into_par_iter()
                .map(|qq| {
                    let mut acc = [F256Unreduced::ZERO; 8];
                    for q in (qq * qc)..((qq + 1) * qc).min(n_quads) {
                        quad(q, &mut acc);
                    }
                    acc
                })
                .reduce(|| [F256Unreduced::ZERO; 8], xor_acc8)
        } else {
            const KC: usize = 1 << 12;
            (0..n_quads * blk.div_ceil(KC))
                .into_par_iter()
                .map(|item| {
                    let chunks_per_quad = blk.div_ceil(KC);
                    let (q, kc) = (item / chunks_per_quad, item % chunks_per_quad);
                    let i0 = 4 * q * blk;
                    let mut acc = [F256Unreduced::ZERO; 8];
                    for k in (kc * KC)..((kc + 1) * KC).min(blk) {
                        let i = i0 + k;
                        let fq = [
                            packed_witness[i],
                            packed_witness[i + blk],
                            packed_witness[i + 2 * blk],
                            packed_witness[i + 3 * blk],
                        ];
                        let bq = [bs(i), bs(i + blk), bs(i + 2 * blk), bs(i + 3 * blk)];
                        lookahead_accum_group(&fq, &bq, &mut acc);
                    }
                    acc
                })
                .reduce(|| [F256Unreduced::ZERO; 8], xor_acc8)
        };
        let (round0, la) = lookahead_finish(acc);
        if trace {
            eprintln!(
                "  [open_batch] combine (seeded EqPoint + lookahead, L={l}): {:6.2} ms",
                t.elapsed().as_secs_f64() * 1e3
            );
        }
        return CombinedClaim {
            ring_switches: Vec::new(),
            batching_nonces,
            b_combined: Vec::new(),
            eq_basis: Some((lo, hi, n_lo)),
            eq_gamma: Some(gammas_pd[0]),
            target_combined,
            round0_prime: (round0.u_0, round0.u_2),
            round1_lookahead: Some(la),
        };
    }

    // Past the special path, EqPoint must not appear — the loops below
    // would silently drop its contribution.
    assert!(
        packed_direct
            .iter()
            .all(|pd| !matches!(pd.eq_ind, DirectEqInd::EqPoint(_))),
        "EqPoint claims are only supported alone, with no RS claims"
    );

    // Fast path (compression-proof open: claims ab, c; also chain/merkle): every
    // RS claim is a fused DeferredDense fold and no DENSE packed-direct claim
    // needs the per-element combine. Fold all claims block-by-block straight into
    // b_combined — each claim's `e_hi` hoisted once per block, exactly as in
    // `fold_b128_elems_split` — and fuse the round-0 prime in the same pass.
    // Neither the per-claim `γ_k·B_k` buffer nor a combine readback is ever
    // materialized (saves ~2·L writes + 2·L reads of the 2^(m-7) basis).
    //
    // SPARSE packed-direct claims (the chain/merkle I/O claim) do NOT disable
    // this path: they're scatter-added onto b_combined after the fold (with an
    // incremental round-0 prime adjustment), so they only require
    // `pd_dense.is_empty()`, not `packed_direct.is_empty()`. This keeps the two
    // big ab/c claims on the fused fold instead of materializing them.
    let use_fast =
        !rs_deferred.is_empty() && rs_deferred.len() == rs_results.len() && pd_dense.is_empty();

    // ---- Build b_combined (γ-weighted sum of all rs_eq_ind + eq_ind) and the
    //      round-0 prime (u_0, u_2 over packed_witness · b_combined).
    let mut b_combined: Vec<F128> = take_f128(l);

    // The combine is compute-bound (open_combine_probe: ~4.3 ms traffic floor
    // vs ~18 ms total at m=30 on 4 P-threads), and its flat block-parallel
    // shape drains cleanly around slow cores — run it on the all-core (P+E)
    // pool (−29% on the probe on 4P+4E; a wash-to-slight-loss on 10P+4E, so
    // gated on [`crate::ecore_rich_topology`], `FLOCK_ALLCORE=1` overrides).
    // PCS_COMBINE_PCORES_ONLY=1 keeps it on the caller's pool (A/B toggle).
    // Thread count never changes the output bits: every slot is written
    // deterministically and the prime is an XOR reduction (associative +
    // commutative, exact).
    let combine_all_cores = var("PCS_COMBINE_PCORES_ONLY").is_err() && ecore_rich_topology();
    // The fast path's block tail can also accumulate Ligerito's round-1
    // message coefficients (groups of 4; +1 unreduced mul per slot) — the
    // round-0 prime falls out of the same accumulators. Sparse post-combine
    // scatter-adds no longer disable this: the coefficients are linear in
    // the basis, so each scattered delta corrects them below. See
    // `CombinedClaim`.
    let want_lookahead = use_fast;
    let mut round1_lookahead: Option<FoldLookahead> = None;
    let b_combined_ref = &mut b_combined;
    let la_ref = &mut round1_lookahead;
    let mut combine = || {
        if use_fast {
            let b = rs_deferred[0].0.len(); // eq_lo.len(); shared across claims (same split)
            debug_assert!(b >= 2 && b.is_multiple_of(2));
            debug_assert!(rs_deferred.iter().all(|d| d.0.len() == b));
            // Fold one block of every claim into `out_block`; returns true when
            // the block is DEAD (stored as zeros — no prime or lookahead terms
            // to accumulate).
            let fold_block = |hi: usize, out_block: &mut [F128]| -> bool {
                // DEAD-BLOCK SKIP. A claim's `e_hi = eq_hi[hi]` is exactly
                // `F128::ZERO` on every block whose high address bits fall
                // outside the claim's own support — which is what a claim point
                // with FROZEN high coordinates produces (`build_eq` of a
                // zero coord kills half the tensor). When EVERY claim's factor
                // vanishes on a block, so does `b_combined` there:
                // `fold_one_slot(0, table) = Σ_k table[k·256 + 0]`, and
                // `build_fold_byte_table` fills `value = 0` with the empty sum,
                // so it is exactly zero. Storing zeros directly is therefore
                // bit-identical to folding, and skips 16 table lookups +
                // 15 XORs + a GF multiply per word.
                //
                // This is where the multi-table design's DISJOINTNESS pays: the
                // boolean class's claim points carry `M − M_bool` frozen-zero
                // high coordinates, so their basis vanishes on the whole
                // element region AND the inter-class gap — half the address
                // space or more. The element class's claims are `Sparse` and
                // only ever touch their own subcube (scatter-added below), so
                // after this skip the combine costs `O(2^M_bool + element
                // support)` rather than `O(2^M)`.
                //
                // Zeros still have to be STORED, not skipped: the
                // virtual-opening sumcheck's round-0 fold reads `b_combined`
                // densely when the witness support is not sparse.
                if rs_deferred.iter().all(|d| d.1[hi] == F128::ZERO) {
                    out_block.fill(F128::ZERO);
                    // The prime's (and lookahead's) terms here are `a·0`.
                    return true;
                }
                // Accumulate each claim's block: first claim writes, rest add.
                // `e_hi` is read once per claim per block, then swept over eq_lo.
                for (ci, (eq_lo, eq_hi, table, _)) in rs_deferred.iter().enumerate() {
                    let e_hi = eq_hi[hi];
                    if ci == 0 {
                        for (slot, &lo) in out_block.iter_mut().zip(eq_lo.iter()) {
                            *slot = fold_one_slot(lo * e_hi, table);
                        }
                    } else {
                        for (slot, &lo) in out_block.iter_mut().zip(eq_lo.iter()) {
                            *slot += fold_one_slot(lo * e_hi, table);
                        }
                    }
                }
                false
            };
            if want_lookahead && b.is_multiple_of(4) {
                // Fused prime + round-1 lookahead tail (groups of 4).
                let acc = b_combined_ref
                    .par_chunks_mut(b)
                    .enumerate()
                    .map(|(hi, out_block)| {
                        if fold_block(hi, out_block) {
                            return [F256Unreduced::ZERO; 8];
                        }
                        let base = hi * b;
                        let mut acc = [F256Unreduced::ZERO; 8];
                        for g in 0..(b / 4) {
                            let i = 4 * g;
                            let fq = [
                                packed_witness[base + i],
                                packed_witness[base + i + 1],
                                packed_witness[base + i + 2],
                                packed_witness[base + i + 3],
                            ];
                            let bq = [
                                out_block[i],
                                out_block[i + 1],
                                out_block[i + 2],
                                out_block[i + 3],
                            ];
                            lookahead_accum_group(&fq, &bq, &mut acc);
                        }
                        acc
                    })
                    .reduce(|| [F256Unreduced::ZERO; 8], xor_acc8);
                let (msg, la) = lookahead_finish(acc);
                *la_ref = Some(la);
                (msg.u_0, msg.u_2)
            } else {
                let (u0, u2) = b_combined_ref
                    .par_chunks_mut(b)
                    .enumerate()
                    .map(|(hi, out_block)| {
                        if fold_block(hi, out_block) {
                            return (F256Unreduced::ZERO, F256Unreduced::ZERO);
                        }
                        // Round-0 prime over this block's pairs (b is even, base is
                        // even). Unreduced 256-bit accumulation, one reduction at
                        // the very end (XOR-linear, bit-identical to reducing per
                        // term).
                        let base = hi * b;
                        let mut u0 = F256Unreduced::ZERO;
                        let mut u2 = F256Unreduced::ZERO;
                        for t in 0..(b / 2) {
                            let s0 = out_block[2 * t];
                            let s1 = out_block[2 * t + 1];
                            let a0 = packed_witness[base + 2 * t];
                            let a1 = packed_witness[base + 2 * t + 1];
                            u0 ^= a0.mul_unreduced(s0);
                            u2 ^= (a0 + a1).mul_unreduced(s0 + s1);
                        }
                        (u0, u2)
                    })
                    .reduce(
                        || (F256Unreduced::ZERO, F256Unreduced::ZERO),
                        |(x0, x2), (y0, y2)| (x0 ^ y0, x2 ^ y2),
                    );
                (u0.reduce(), u2.reduce())
            }
        } else {
            // General path (mixed / sparse / packed-direct): materialize any
            // deferred-dense claims (parallel block fold), then the per-element
            // combine over all dense buffers + packed-direct, matching the
            // original behavior.
            let materialized: Vec<Vec<F128>> = rs_results
                .iter()
                .filter_map(|(_, o)| match &o.rs_eq_ind {
                    RsEqInd::DeferredDense {
                        eq_lo,
                        eq_hi,
                        table,
                    } => Some(fold_b128_from_table(eq_lo, eq_hi, table)),
                    _ => None,
                })
                .collect();
            let mut rs_dense_all: Vec<&[F128]> = rs_baked.clone();
            rs_dense_all.extend(materialized.iter().map(|v| v.as_slice()));
            if rs_dense_all.is_empty() && pd_dense.is_empty() {
                // All-sparse open (e.g. the permutation check's five pinned-coord
                // claims): the per-element loop below would write zeros into every
                // slot and accumulate `a·0` into the prime. Skip straight to the
                // memset — the sparse scatter-adds that follow supply the whole of
                // `b_combined`, and the prime they fold in is the whole prime.
                b_combined_ref.par_chunks_mut(1 << 13).for_each(|c| {
                    c.fill(F128::ZERO);
                });
                return (F128::ZERO, F128::ZERO);
            }
            let prime = b_combined_ref
                .par_chunks_mut(2)
                .enumerate()
                .map(|(i, chunk)| {
                    let mut b0 = F128::ZERO;
                    let mut b1 = F128::ZERO;
                    for v in rs_dense_all.iter() {
                        b0 += v[2 * i];
                        b1 += v[2 * i + 1];
                    }
                    for (v, g) in pd_dense.iter() {
                        b0 += *g * v[2 * i];
                        b1 += *g * v[2 * i + 1];
                    }
                    chunk[0] = b0;
                    chunk[1] = b1;
                    let a0 = packed_witness[2 * i];
                    let a1 = packed_witness[2 * i + 1];
                    (a0 * b0, (a0 + a1) * (b0 + b1))
                })
                .reduce(
                    || (F128::ZERO, F128::ZERO),
                    |(x0, x2), (y0, y2)| (x0 + y0, x2 + y2),
                );
            for v in materialized {
                give_f128(v);
            }
            prime
        }
    };
    let (mut round0_u0, mut round0_u2) = if combine_all_cores {
        all_core_pool().install(combine)
    } else {
        combine()
    };
    let mut adjust_prime_for_delta = |idx: usize, delta: F128| {
        let pair = idx / 2;
        let a0 = packed_witness[2 * pair];
        let a1 = packed_witness[2 * pair + 1];
        if idx & 1 == 0 {
            round0_u0 += a0 * delta;
        }
        round0_u2 += (a0 + a1) * delta;
    };
    // Post-combine b_combined mutations. The round-1 lookahead coefficients
    // are LINEAR in the basis, so every scattered delta corrects them with
    // its own quad contribution (`bq` = the delta at one slot, `fq` = the
    // quad's witness words — 8 unreduced muls per live entry); the prime
    // keeps its existing incremental adjustment.
    let mut la_acc = [F256Unreduced::ZERO; 8];
    let la_active = round1_lookahead.is_some();
    let la_correct = |idx: usize, delta: F128, acc: &mut [F256Unreduced; 8]| {
        let g = idx & !3usize;
        let fq = [
            packed_witness[g],
            packed_witness[g + 1],
            packed_witness[g + 2],
            packed_witness[g + 3],
        ];
        let mut bq = [F128::ZERO; 4];
        bq[idx & 3] = delta;
        lookahead_accum_group(&fq, &bq, acc);
    };
    for (_, output) in rs_results.iter() {
        if let RsEqInd::Sparse { entries, .. } = &output.rs_eq_ind {
            for &(idx, val) in entries {
                b_combined[idx] += val;
                adjust_prime_for_delta(idx, val);
                if la_active {
                    la_correct(idx, val, &mut la_acc);
                }
            }
        }
    }
    for (pd, g) in packed_direct.iter().zip(gammas_pd.iter()) {
        if let DirectEqInd::Sparse(eq) = &pd.eq_ind {
            // Scatter-add the sparse claim and fold its round-0 prime
            // contribution in the SAME pass (O(live positions)), instead of a
            // full O(L) re-pass over b_combined. The prime is linear in
            // b_combined, so the delta from scattering `g·eq` equals
            // Σ adjust_prime_for_delta(idx, g·val) over the live positions.
            let (du0, du2) = sparse_scatter_add_parallel(&mut b_combined, packed_witness, eq, *g);
            round0_u0 += du0;
            round0_u2 += du2;
            if la_active {
                for c in 0..eq.live_tensor.len() {
                    la_correct(eq.scatter_idx(c), *g * eq.live_tensor[c], &mut la_acc);
                }
            }
        }
    }
    if let Some(la) = round1_lookahead.as_mut() {
        // The group kernel's message slots duplicate the prime deltas
        // already applied above — only the coefficient half is consumed.
        let (_, delta) = lookahead_finish(la_acc);
        la.add(&delta);
    }
    if trace {
        eprintln!(
            "  [open_batch] combine rs_eq_ind (L={}, rs×{}, pd×{}): {:6.2} ms",
            l,
            n_rs,
            n_pd,
            t.elapsed().as_secs_f64() * 1e3
        );
    }

    let ring_switches = rs_results
        .into_iter()
        .map(|(p, o)| {
            // The per-claim rs_eq_ind (L F128s) dies here — recycle it.
            if let RsEqInd::Dense(v) = o.rs_eq_ind {
                give_f128(v);
            }
            p
        })
        .collect();
    CombinedClaim {
        ring_switches,
        batching_nonces,
        b_combined,
        eq_basis: None,
        eq_gamma: None,
        target_combined,
        round0_prime: (round0_u0, round0_u2),
        round1_lookahead,
    }
}

/// Parallel sparse scatter-add: `b_combined[scatter_idx(c)] += gamma * eq.live_tensor[c]`
/// for every `c`. Partitions `c`-space across rayon threads; since
/// [`SparseEqTensor::scatter_idx`] is monotonic in `c` (live_positions sorted
/// ascending), each thread's scattered indices fall in a contiguous, disjoint
/// range of `b_combined`. Splits `b_combined` at the chunk boundaries via
/// `split_at_mut`, then writes scatter-adds into the disjoint mutable slices —
/// safe rust, no atomics.
/// Scatter-add `gamma · eq` into `b_combined` and return the resulting
/// round-0 prime delta `(Δu0, Δu2)`. Because the prime is linear in
/// `b_combined`, adding `delta = gamma·val` at index `idx` changes the prime by
/// `Δu0 += a0·delta` (if `idx` even) and `Δu2 += (a0+a1)·delta`, where
/// `a0 = packed_witness[2·pair]`, `a1 = packed_witness[2·pair+1]`,
/// `pair = idx/2`. Computing it here (O(live positions)) avoids a full O(L)
/// re-pass over `b_combined` at the call site.
fn sparse_scatter_add_parallel(
    b_combined: &mut [F128],
    packed_witness: &[F128],
    eq: &SparseEqTensor,
    gamma: F128,
) -> (F128, F128) {
    let c_total = eq.live_tensor.len();
    if c_total == 0 {
        return (F128::ZERO, F128::ZERO);
    }
    let n_threads = current_num_threads().max(1);
    let c_per_chunk = c_total.div_ceil(n_threads).max(1);
    let actual_n_chunks = c_total.div_ceil(c_per_chunk);

    // Boundaries in `b_combined` index space. `b_boundaries[i]` is where chunk
    // `i` starts. `b_boundaries[i+1] − b_boundaries[i]` is chunk `i`'s slice
    // length. The last chunk extends to `b_combined.len()` to absorb any tail
    // positions beyond the maximum scatter idx (those contain only dense
    // contributions from the parallel pass).
    let b_boundaries: Vec<usize> = (0..=actual_n_chunks)
        .map(|i| {
            if i == 0 {
                0
            } else if i == actual_n_chunks {
                b_combined.len()
            } else {
                eq.scatter_idx(i * c_per_chunk)
            }
        })
        .collect();
    debug_assert!(b_boundaries.windows(2).all(|w| w[0] <= w[1]));

    // Disjoint mutable slices via repeated split_at_mut.
    let mut remaining: &mut [F128] = b_combined;
    let mut slices: Vec<&mut [F128]> = Vec::with_capacity(actual_n_chunks);
    for i in 1..actual_n_chunks {
        let split_at = b_boundaries[i] - b_boundaries[i - 1];
        let (left, right) = remaining.split_at_mut(split_at);
        slices.push(left);
        remaining = right;
    }
    slices.push(remaining);
    debug_assert_eq!(slices.len(), actual_n_chunks);

    slices
        .into_par_iter()
        .enumerate()
        .map(|(t, slice)| {
            let c_lo = t * c_per_chunk;
            let c_hi = ((t + 1) * c_per_chunk).min(c_total);
            let b_lo = b_boundaries[t];
            let mut du0 = F128::ZERO;
            let mut du2 = F128::ZERO;
            for c in c_lo..c_hi {
                let val = eq.live_tensor[c];
                let idx = eq.scatter_idx(c);
                let delta = gamma * val;
                slice[idx - b_lo] += delta;
                // Round-0 prime delta for this scattered position.
                let pair = idx / 2;
                let a0 = packed_witness[2 * pair];
                let a1 = packed_witness[2 * pair + 1];
                if idx & 1 == 0 {
                    du0 += a0 * delta;
                }
                du2 += (a0 + a1) * delta;
            }
            (du0, du2)
        })
        .reduce(
            || (F128::ZERO, F128::ZERO),
            |(x0, x2), (y0, y2)| (x0 + y0, x2 + y2),
        )
}

/// Verifier reference to a packed-direct claim: the multilinear point at
/// which `ẑ_packed` was claimed equal to `value`. The verifier owns the data
/// (it appears in the public statement of whatever produced the claim, e.g.
/// the chain shift sumcheck output).
#[derive(Clone, Copy, Debug)]
pub struct PackedDirectClaimRef<'a> {
    pub point: &'a [F128],
    pub value: F128,
}

/// Verify a mixed-claim batched opening (mirror of
/// [`open_batch_mixed_ligerito_with_precomputed_s_hat_v_and_grinding`]). Uses
/// `ring_switch::verify_succinct` per claim (no dense `rs_eq_ind`
/// materialization), then drives the succinct recursive Ligerito verifier,
/// evaluating the combined basis only at the residual point, with the
/// matching PCS-transport PoW checks.
#[allow(clippy::too_many_arguments)]
pub fn verify_opening_batch_ligerito_mixed_with_grinding<Ch: Challenger>(
    commitment: &Commitment,
    claims: &[F128],
    skip_weights: &[&[F128]],
    x_outers: &[&[F128]],
    packed_direct: &[PackedDirectClaimRef<'_>],
    proof: &BatchOpeningProofLigerito,
    lig_config: &VerifierConfig,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
) -> Result<(), PcsError> {
    let n_rs = claims.len();
    let n_pd = packed_direct.len();
    assert_eq!(skip_weights.len(), n_rs);
    assert_eq!(x_outers.len(), n_rs);
    if proof.ring_switches.len() != n_rs {
        return Err(PcsError::RingSwitch(RingSwitchError::MalformedProof));
    }
    assert!(n_rs + n_pd > 0);
    let batch_bits = grinding.claim_batch_bits_for(n_rs + n_pd);
    let expected_batch_nonces = usize::from(batch_bits != 0);
    if proof.batching_nonces.len() != expected_batch_nonces {
        return Err(PcsError::RingSwitch(RingSwitchError::InvalidGrinding));
    }
    // Lane-major (integer-lane) commitments: supported only for the merged
    // inner-open configuration (packed-direct claims only). The RS claims'
    // residual machinery has no rotated-point path; reject rather than
    // mis-evaluate. (The claim shapes are statement-derived, so this is an
    // assert, not an attacker-reachable rejection.)
    let lane_major = commitment.params.num_ntts() < 1usize << lig_config.initial_k;
    assert!(
        !lane_major || n_rs == 0,
        "lane-major mixed verify: packed-direct claims only"
    );

    challenger.observe_label(b"flock-pcs-open-batch-v0");

    // 1. Ring-switch SUCCINCT verify per claim — gets sumcheck_claim and a
    //    length-128 `eq_r_dprime` instead of the dense `rs_eq_ind`. Saves
    //    ~16 MB allocation at m=29.
    let mut rs_outputs = Vec::with_capacity(n_rs);
    for i in 0..n_rs {
        let out = verify_succinct_with_grinding(
            claims[i],
            skip_weights[i],
            x_outers[i],
            &proof.ring_switches[i],
            grinding.ring_switch_bits,
            challenger,
        )
        .map_err(PcsError::RingSwitch)?;
        rs_outputs.push(out);
    }
    // 2. Bind every PD value, then protect the whole mixed linear batching
    // vector with one PoW. A discrepancy is total-degree one in this vector.
    for pd in packed_direct {
        challenger.observe_label(b"flock-pcs-packed-direct-v0");
        challenger.observe_f128(pd.value);
    }
    let gammas = if batch_bits != 0 {
        challenger
            .verify_pow_and_sample_f128_vec(proof.batching_nonces[0], batch_bits, n_rs + n_pd)
            .ok_or(PcsError::RingSwitch(RingSwitchError::InvalidGrinding))?
    } else {
        challenger.sample_f128_vec(n_rs + n_pd)
    };
    let (gammas_rs, gammas_pd) = gammas.split_at(n_rs);

    // 3. target_combined from succinct rs claims + PD values.
    let mut target_combined = F128::ZERO;
    for (out, g) in rs_outputs.iter().zip(gammas_rs.iter()) {
        target_combined += *g * out.sumcheck_claim;
    }
    for (pd, g) in packed_direct.iter().zip(gammas_pd.iter()) {
        target_combined += *g * pd.value;
    }

    // 4. Batch evaluator: returns b_combined at all yr positions in one call.
    //    For RS claims, precompute the ring_switch tensor PREFIX once (over
    //    the ris part) and only re-do the yr_log_n-step suffix per y.
    //    For PD claims, precompute eq prefix factors over ris and finish per y.
    //    For BLAKE3 m=30: ris is 19 dims, yr is 4 dims → 19× prefix reuse.
    let log_n = commitment.params.m - LOG_PACKING;
    let eval_b_residual = |ris: &[F256], yr_log_n: usize| -> Vec<F256> {
        let yr_len = 1usize << yr_log_n;
        let prefix_len = ris.len();

        // ---- RS claim prefixes ----
        let rs_prefixes: Vec<TensorAlgebra256> = rs_outputs
            .iter()
            .zip(x_outers.iter())
            .map(|(_out, x_outer)| {
                // x_outer[1..] has length log_n; we feed only the ris prefix.
                eval_rs_eq_prefix_f256(&x_outer[1..1 + prefix_len], ris)
            })
            .collect();

        // ---- PD claim prefix scalars ----
        // eq(pd.point, point) factors over coordinates; precompute the prefix
        // product. Under a lane-major commitment the fold challenges bind the
        // ROTATED variable order (lane vars — the high dense vars — first),
        // so pair them against the correspondingly rotated claim point.
        let pd_points_rot: Vec<Vec<F128>> = packed_direct
            .iter()
            .map(|pd| {
                if lane_major {
                    rotate_lane_point(pd.point, pd.point.len() - lig_config.initial_k)
                } else {
                    pd.point.to_vec()
                }
            })
            .collect();
        let pd_prefix_scalars: Vec<F256> = pd_points_rot
            .iter()
            .map(|pt| {
                pt[..prefix_len]
                    .iter()
                    .zip(ris)
                    .fold(F256::ONE, |acc, (&p, &r)| {
                        acc * (F256::ONE + F256::from(p) + r)
                    })
            })
            .collect();

        debug_assert!(yr_log_n <= 32, "yr_log_n > 32 not supported by binary path");
        (0..yr_len)
            .into_par_iter()
            .map(|y| {
                let y_bits = y as u32;
                let mut sum = F256::ZERO;
                for (((out, g), x_outer), prefix) in rs_outputs
                    .iter()
                    .zip(gammas_rs.iter())
                    .zip(x_outers.iter())
                    .zip(rs_prefixes.iter())
                {
                    sum += *g
                        * eval_rs_eq_finish_from_prefix_binary_q_f256(
                            prefix,
                            &x_outer[1 + prefix_len..],
                            y_bits,
                            &out.eq_r_dprime,
                        );
                }
                for ((pt, g), prefix_scalar) in pd_points_rot
                    .iter()
                    .zip(gammas_pd.iter())
                    .zip(pd_prefix_scalars.iter())
                {
                    let suffix =
                        pt[prefix_len..]
                            .iter()
                            .enumerate()
                            .fold(F128::ONE, |acc, (j, &p)| {
                                acc * if (y_bits >> j) & 1 == 1 {
                                    p
                                } else {
                                    F128::ONE + p
                                }
                            });
                    sum += *prefix_scalar * (*g * suffix);
                }
                sum
            })
            .collect()
    };

    // 5. Drive ligerito SUCCINCT verifier — eval_b_residual is called ONCE
    //    at the residual check (returns all yr_len values in one batch).
    let ok = recursive_verifier_with_basis_succinct(
        lig_config,
        &proof.ligerito,
        log_n,
        target_combined,
        &commitment.cap,
        commitment.params.num_ntts(),
        eval_b_residual,
        challenger,
    );
    if !ok {
        return Err(PcsError::Ligerito);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
/// Map a point over the LANE-GRID variables to the corresponding point over
/// the dense-stack variables, for a high-bit-lane commit.
///
/// The grid is `q_grid[p·2^k + l] = q[l·D + p]` (`k = initial_k`), i.e.
/// `q_grid`'s low `k` variables are the lane bits that live at the TOP of
/// `q`'s index. As multilinears that is a pure cyclic rotation of the variable
/// vector, so `q̂_grid(x) = q̂(x_k, …, x_{m−1}, x_0, …, x_{k−1})` — rotate the
/// evaluation point left by `k`.
fn rotate_lane_point(point: &[F128], k: usize) -> Vec<F128> {
    debug_assert!(k <= point.len());
    let mut out = Vec::with_capacity(point.len());
    out.extend_from_slice(&point[k..]);
    out.extend_from_slice(&point[..k]);
    out
}

// ───────────────────────────────────────────────────────────────────────────
// The merged jagged and ring-switch opening uses one sumcheck over the dense
// domain. The weight `W[d] = Σ_i Φ_i(eq_row·eq_col at the
// unrank of d)` is simultaneously the ring-switch weight and the
// dense/virtual translation, so the prover's Φ-pass is count-proportional
// (capacity-free). The verifier's twisted-weight evaluation `Ŵ(ρ)` is
// discharged by the batched Frobenius assist, and `q̂(ρ)` by an ordinary
// eq-basis Ligerito opening (a packed-direct claim on the mixed path).
// Handles both power-of-two and integer-lane commitments.
// ───────────────────────────────────────────────────────────────────────────

/// Proof of the merged opening. `merged_rounds` are the dense-domain
/// sumcheck's `(G(1), G(∞))` messages (`m_dense − 7` rounds, LSB first);
/// `q_eval = q̂(ρ)`; the Frobenius assist carries and proves the twisted
/// evaluation `V = Ŵ(ρ)`; `inner` is the eq-basis Ligerito opening of
/// `q̂(ρ)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedOpenProof {
    pub ring_switches: Vec<RingSwitchProof>,
    /// PoW witnesses for the outer opening-claim batching coefficients:
    /// ring-switched claims first, then packed-direct claims.
    #[serde(default)]
    pub batching_nonces: Vec<u64>,
    pub merged_rounds: Vec<(F128, F128)>,
    /// PoW witnesses for the dense merged sumcheck, in round order.
    #[serde(default)]
    pub merged_round_nonces: Vec<u64>,
    pub q_eval: F128,
    pub frobenius: MultipointTwistedProof,
    pub inner: BatchOpeningProofLigerito,
}

/// γ-combine the packed-direct claims into merged-column scalar groups,
/// keyed by shared row point in first-occurrence order: `Σᵢ γᵢ·eq_rowᵢ(row)·
/// eq_colᵢ(col) = eq_row(row)·(Σᵢ γᵢ·eq_colᵢ(col))`, so a group costs ONE
/// sweep wherever it is consumed — the merged `W` build, the multipoint
/// protocol (one dual value per group), and the anchor assist. Built
/// identically on both sides (the prover's claim order is the verifier's).
///
/// A Boolean column point's eq table is a one-hot indicator, so its
/// γ-contribution is a single scatter — the gather claims' column parts are
/// exactly this (`bits(word_col) ‖ bits(slot_prefix)`), and building a
/// `2^k_cols` table per claim is `k_cols`-exponential waste (~2^20 per
/// claim in a large composite registry). Random column points (the element
/// claims) keep the dense build. Value-identical either way.
fn scalar_claim_groups<'a>(
    points: impl Iterator<Item = &'a [F128]>,
    gammas: &[F128],
    n_log: usize,
    k_cols: usize,
) -> Vec<(&'a [F128], Vec<F128>)> {
    let mut groups: Vec<(&[F128], Vec<F128>)> = Vec::new();
    for (point, &g) in points.zip(gammas) {
        assert_eq!(
            point.len(),
            n_log + k_cols,
            "packed-direct point/row/col split mismatch (no skip coordinate)"
        );
        let (zr, zc) = (&point[..n_log], &point[n_log..]);
        let hot: Option<usize> = zc.iter().enumerate().try_fold(0usize, |acc, (i, &x)| {
            if x == F128::ZERO {
                Some(acc)
            } else if x == F128::ONE {
                Some(acc | (1 << i))
            } else {
                None
            }
        });
        let cols = match groups.iter_mut().find(|(r, _)| *r == zr) {
            Some((_, cols)) => cols,
            None => {
                groups.push((zr, vec![F128::ZERO; 1 << k_cols]));
                &mut groups.last_mut().unwrap().1
            }
        };
        match hot {
            Some(h) => cols[h] += g,
            None => {
                for (dst, e) in cols.iter_mut().zip(build_eq_table(zc)) {
                    *dst += g * e;
                }
            }
        }
    }
    groups
}

/// One ring-switch claim's inputs to the block-first phase.
struct BlockFirstClaim<'a> {
    z_row: &'a [F128],
    z_col: &'a [F128],
    /// `linearized_coefficients` of its γ-scaled fold table.
    coeffs: &'a [F128],
    /// Column bit-bank at `z_row`:
    /// `bank[c·128 + b] = Σ_r eq(z_row, r)·bit_b(q[c·2^n + r])`.
    bank: &'a [F128],
}

// Small openings retain the original dense lookup. Above 2^20 rows, a
// per-claim row table becomes a capacity problem (11 tables at 2^30 rows
// occupy 176 GiB). Reuse the ring-switch tensor split: at 2^30 rows each
// claim instead holds two 2^15 tables, with identical field values.
enum MergedRowEq {
    Dense(Vec<F128>),
    Split {
        lo: Vec<F128>,
        hi: Vec<F128>,
        n_lo: usize,
    },
}

#[cfg(test)]
thread_local! {
    static MERGED_SPLIT_ROWS_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static MERGED_ROW_GROUPING_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static PAIRED_BITBANKS_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static SPARSE_BITBANKS_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static CONSTANT_BITBANKS_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static DIRECT_BLOCK_PARALLEL_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static DIRECT_WP_SIMD_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static WP_UNINIT_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
}

fn wp_uninit_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = WP_UNINIT_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_WP_UNINIT").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_wp_uninit_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            WP_UNINIT_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(WP_UNINIT_OVERRIDE.replace(Some(enabled)));
    f()
}

// Opt-in pending the separate native end-to-end ablation. Only the direct
// contribution to rectangular W' changes its evaluation order.
fn direct_wp_simd_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = DIRECT_WP_SIMD_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_DIRECT_WP_SIMD").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_direct_wp_simd_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            DIRECT_WP_SIMD_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(DIRECT_WP_SIMD_OVERRIDE.replace(Some(enabled)));
    f()
}

fn direct_block_parallel_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = DIRECT_BLOCK_PARALLEL_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_NO_DIRECT_BLOCK_PARALLEL").is_none()
}

#[cfg(test)]
fn with_direct_block_parallel_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            DIRECT_BLOCK_PARALLEL_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(DIRECT_BLOCK_PARALLEL_OVERRIDE.replace(Some(enabled)));
    f()
}

fn constant_bitbanks_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = CONSTANT_BITBANKS_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_NO_CONSTANT_BITBANKS").is_none()
}

#[cfg(test)]
fn with_constant_bitbanks_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            CONSTANT_BITBANKS_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(CONSTANT_BITBANKS_OVERRIDE.replace(Some(enabled)));
    f()
}

fn sparse_bitbanks_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = SPARSE_BITBANKS_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_NO_SPARSE_BITBANKS").is_none()
}

#[cfg(test)]
fn with_sparse_bitbanks_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            SPARSE_BITBANKS_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(SPARSE_BITBANKS_OVERRIDE.replace(Some(enabled)));
    f()
}

fn paired_bitbanks_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = PAIRED_BITBANKS_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_NO_PAIRED_BITBANKS").is_none()
}

#[cfg(test)]
fn with_paired_bitbanks_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            PAIRED_BITBANKS_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(PAIRED_BITBANKS_OVERRIDE.replace(Some(enabled)));
    f()
}

// Both systems use the same native PCS default. The environment switch is
// only an engineering ablation: all messages and challenge order agree.
fn merged_row_grouping_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = MERGED_ROW_GROUPING_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_NO_MERGED_ROW_GROUPING").is_none()
}

#[cfg(test)]
fn with_merged_row_grouping_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            MERGED_ROW_GROUPING_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(MERGED_ROW_GROUPING_OVERRIDE.replace(Some(enabled)));
    f()
}

#[cfg(test)]
fn with_merged_split_rows_override<T>(split: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            MERGED_SPLIT_ROWS_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(MERGED_SPLIT_ROWS_OVERRIDE.replace(Some(split)));
    f()
}

impl MergedRowEq {
    fn new(point: &[F128]) -> Self {
        let split = point.len() > 20;
        #[cfg(test)]
        let split = MERGED_SPLIT_ROWS_OVERRIDE.get().unwrap_or(split);
        if split {
            let n_lo = point.len() / 2;
            let (lo, hi) = ring_switch::build_eq_split(point, n_lo);
            Self::Split { lo, hi, n_lo }
        } else {
            Self::Dense(build_eq_table(point))
        }
    }

    #[inline]
    fn at(&self, row: usize) -> F128 {
        match self {
            Self::Dense(table) => table[row],
            Self::Split { lo, hi, n_lo } => lo[row & (lo.len() - 1)] * hi[row >> n_lo],
        }
    }
}

/// A contiguous field dot product, with one reduction per row chunk. The
/// SIMD accumulator has exactly the same XOR sum of unreduced products as
/// the scalar tail; no floating-point reassociation is involved.
fn direct_block_dot(words: &[F128], weights: &[F128]) -> F128 {
    assert_eq!(words.len(), weights.len());
    let mut sum = F256Unreduced::ZERO;
    #[allow(unused_mut)]
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    // SAFETY: the target-feature gate covers every operation, and both
    // unaligned loads contain four values inside their equally sized slices.
    unsafe {
        use crate::field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu};
        let mut wide = WideGhashX4::zero();
        while i + 4 <= words.len() {
            wide.mul_acc(
                f128x4_loadu(words.as_ptr().add(i)),
                f128x4_loadu(weights.as_ptr().add(i)),
            );
            i += 4;
        }
        sum ^= wide.fold();
    }
    for (&word, &weight) in words[i..].iter().zip(&weights[i..]) {
        sum ^= word.mul_unreduced(weight);
    }
    sum.reduce()
}

/// Column statistics before applying a scalar group's column coefficients.
/// Parallelize rows as well as columns: a four-column union must still use
/// the whole Rayon pool. Split tensors factor out the common high weight
/// once per chunk instead of multiplying it at every word. Only the existing
/// row-weight tables and the O(number of columns) result are retained.
fn direct_block_statistics(
    q: &[F128],
    n_log: usize,
    k_cols: usize,
    live_cols: usize,
    eq_row: &MergedRowEq,
) -> Vec<F128> {
    let n_rows = 1usize << n_log;
    let parallel = direct_block_parallel_enabled();
    (0..1usize << k_cols)
        .into_par_iter()
        .map(|c| {
            if c >= live_cols {
                return F128::ZERO;
            }
            let column = &q[c * n_rows..(c + 1) * n_rows];
            if !parallel {
                return column
                    .iter()
                    .enumerate()
                    .fold(F128::ZERO, |acc, (r, &x)| acc + x * eq_row.at(r));
            }
            match eq_row {
                MergedRowEq::Dense(table) => column
                    .par_chunks(8192)
                    .zip(table.par_chunks(8192))
                    .map(|(words, weights)| direct_block_dot(words, weights))
                    .reduce(|| F128::ZERO, |a, b| a + b),
                MergedRowEq::Split { lo, hi, n_lo } => {
                    // Both sizes are powers of two, so a chunk is wholly
                    // inside one high-weight block, including n_lo=0.
                    let chunk = lo.len().min(8192);
                    column
                        .par_chunks(chunk)
                        .enumerate()
                        .map(|(i, words)| {
                            let row = i * chunk;
                            let lo_start = row & (lo.len() - 1);
                            direct_block_dot(words, &lo[lo_start..lo_start + words.len()])
                                * hi[row >> n_lo]
                        })
                        .reduce(|| F128::ZERO, |a, b| a + b)
                }
            }
        })
        .collect()
}

/// Add one scaled, contiguous row-weight segment without a temporary array.
/// The four-lane field multiply returns four independently reduced products,
/// exactly matching the portable multiply-and-XOR loop.
fn direct_wp_scaled_add(dst: &mut [F128], weights: &[F128], scale: F128) {
    assert_eq!(dst.len(), weights.len());
    if scale.is_zero() {
        return;
    }
    #[allow(unused_mut)]
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    // SAFETY: target features cover all intrinsics; each unaligned load and
    // store spans four in-bounds values of the equally sized slices. The
    // exclusive output borrow cannot alias the immutable weight slice.
    unsafe {
        use crate::field::gf2_128::x86_64::{f128x4_loadu, f128x4_set, ghash_mul_x4};
        use core::arch::x86_64::{__m512i, _mm512_storeu_si512, _mm512_xor_si512};
        let factor = f128x4_set(scale, scale, scale, scale);
        while i + 4 <= dst.len() {
            let product = ghash_mul_x4(f128x4_loadu(weights.as_ptr().add(i)), factor);
            let value = _mm512_xor_si512(f128x4_loadu(dst.as_ptr().add(i)), product);
            _mm512_storeu_si512(dst.as_mut_ptr().add(i).cast::<__m512i>(), value);
            i += 4;
        }
    }
    for (slot, &weight) in dst[i..].iter_mut().zip(&weights[i..]) {
        *slot += scale * weight;
    }
}

/// Add direct scalar groups after the block challenges have already been
/// bound. On split rows, s[c0]*hi[u] is constant over a segment, leaving only
/// one multiply per output instead of s[c0]*(lo[v]*hi[u]). All support and
/// column cross terms have already entered s; zero scales can now be skipped.
fn direct_wp_add_groups(
    dst: &mut [F128],
    offset: usize,
    n_log: usize,
    groups: &[(MergedRowEq, [F128; 2])],
) {
    let n_rows = 1usize << n_log;
    assert!(offset <= 2 * n_rows && dst.len() <= 2 * n_rows - offset);
    for (eq_row, scales) in groups {
        let mut i = 0;
        while i < dst.len() {
            let h = offset + i;
            let c0 = h >> n_log;
            let row = h & (n_rows - 1);
            // Tiny row domains can put both c0 values in one worker chunk.
            let mut len = (dst.len() - i).min(n_rows - row);
            match eq_row {
                MergedRowEq::Dense(table) => {
                    direct_wp_scaled_add(&mut dst[i..i + len], &table[row..row + len], scales[c0]);
                }
                MergedRowEq::Split { lo, hi, n_lo } => {
                    let low = row & (lo.len() - 1);
                    len = len.min(lo.len() - low);
                    direct_wp_scaled_add(
                        &mut dst[i..i + len],
                        &lo[low..low + len],
                        scales[c0] * hi[row >> n_lo],
                    );
                }
            }
            i += len;
        }
    }
}

/// First-touch initialization of one exclusively owned W' output chunk.
/// No F128 view is constructed until every slot in this chunk is initialized.
fn initialize_wp_chunk(
    chunk: &mut [core::mem::MaybeUninit<F128>],
    offset: usize,
    n_log: usize,
    eq_rows: &[MergedRowEq],
    tabs: &[[Vec<F128>; 2]],
    pd: &[(MergedRowEq, [F128; 2])],
    direct_wp_simd: bool,
) {
    let n_rows = 1usize << n_log;
    let scalar_pd = if direct_wp_simd { &[][..] } else { pd };
    for (i, slot) in chunk.iter_mut().enumerate() {
        let h = offset + i;
        let c0 = h >> n_log;
        let row = h & (n_rows - 1);
        let mut acc = F128::ZERO;
        for (eq_row, tab) in eq_rows.iter().zip(tabs) {
            acc += ring_switch::fold_one_slot(eq_row.at(row), &tab[c0]);
        }
        for (eq_row, scale) in scalar_pd {
            acc += scale[c0] * eq_row.at(row);
        }
        slot.write(acc);
    }
    if direct_wp_simd {
        // SAFETY: the preceding loop wrote every slot, MaybeUninit<F128>
        // has the same size/alignment as F128, and the worker exclusively
        // owns this slice. A panic before here never reads unwritten slots.
        let initialized = unsafe {
            core::slice::from_raw_parts_mut(chunk.as_mut_ptr().cast::<F128>(), chunk.len())
        };
        direct_wp_add_groups(initialized, offset, n_log, pd);
    }
}

/// `x^(2^j)`.
#[inline]
fn frob(mut x: F128, j: usize) -> F128 {
    for _ in 0..j {
        x = x.square();
    }
    x
}

/// Round message of the product sumcheck from rank-1 terms `Σ_t A_t(e)·S_t(e)`
/// (LSB-first pairs): `(Σ_odd A·S, Σ (A0+A1)(S0+S1))`.
fn round_message_terms(terms: &[(Vec<F128>, Vec<F128>)]) -> (F128, F128) {
    let mut g_one = F128::ZERO;
    let mut g_inf = F128::ZERO;
    for (a, s) in terms {
        for i in 0..a.len() / 2 {
            let (a0, a1, s0, s1) = (a[2 * i], a[2 * i + 1], s[2 * i], s[2 * i + 1]);
            g_one += a1 * s1;
            g_inf += (a0 + a1) * (s0 + s1);
        }
    }
    (g_one, g_inf)
}

/// `v[i] ← v[2i] + r·(v[2i] + v[2i+1])`, halving `v`.
fn fold_pairs_in_place(v: &mut Vec<F128>, r: F128) {
    let half = v.len() / 2;
    for i in 0..half {
        let (x0, x1) = (v[2 * i], v[2 * i + 1]);
        v[i] = x0 + r * (x0 + x1);
    }
    v.truncate(half);
}

/// Round message of the product sumcheck over a dense pair (see
/// `round_message_terms`).
fn round_message_pairs(a: &[F128], b: &[F128]) -> (F128, F128) {
    use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice};
    a.par_chunks(4096)
        .zip(b.par_chunks(4096))
        .map(|(ac, bc)| {
            let mut g_one = F128::ZERO;
            let mut g_inf = F128::ZERO;
            for i in 0..ac.len() / 2 {
                let (a0, a1, b0, b1) = (ac[2 * i], ac[2 * i + 1], bc[2 * i], bc[2 * i + 1]);
                g_one += a1 * b1;
                g_inf += (a0 + a1) * (b0 + b1);
            }
            (g_one, g_inf)
        })
        .reduce(|| (F128::ZERO, F128::ZERO), |x, y| (x.0 + y.0, x.1 + y.1))
}

/// Reference column bit-bank of the dense stack at `z_row` (the union prover
/// supplies these from its stripe kernels; this is the slow oracle).
fn column_bitbank_reference(q: &[F128], z_row: &[F128], n_log: usize, k_cols: usize) -> Vec<F128> {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    let eq_row = MergedRowEq::new(z_row);
    let n_rows = 1usize << n_log;
    (0..1usize << k_cols)
        .into_par_iter()
        .flat_map_iter(|c| {
            let mut acc = vec![F128::ZERO; 1 << LOG_PACKING];
            for (r, w) in q[c * n_rows..(c + 1) * n_rows].iter().enumerate() {
                let e = eq_row.at(r);
                let mut lo = w.lo;
                while lo != 0 {
                    acc[lo.trailing_zeros() as usize] += e;
                    lo &= lo - 1;
                }
                let mut hi = w.hi;
                while hi != 0 {
                    acc[64 + hi.trailing_zeros() as usize] += e;
                    hi &= hi - 1;
                }
            }
            acc
        })
        .collect()
}

/// The same per-column bank through the native split-tensor subset-sum
/// kernel. Each column parallelizes over row blocks, so four word columns
/// can still use the entire pool. The tables depend on this proof's row
/// challenge and are built online; no full row tensor or witness copy is
/// created. Tiny domains retain the scalar oracle's shape support.
fn column_bitbank_split(q: &[F128], z_row: &[F128], n_log: usize, k_cols: usize) -> Vec<F128> {
    assert_eq!(z_row.len(), n_log);
    let n_rows = 1usize << n_log;
    let n_cols = 1usize << k_cols;
    assert!(q.len() >= n_rows * n_cols);
    if n_log < 4 {
        return column_bitbank_reference(q, z_row, n_log, k_cols);
    }
    let n_lo = ring_switch::split_n_lo(n_log);
    let (lo, hi) = ring_switch::build_eq_split(z_row, n_lo);
    let padding = PaddingSpec::dense(n_log + LOG_PACKING);
    let mut bank = Vec::with_capacity(n_cols << LOG_PACKING);
    for column in q[..n_rows * n_cols].chunks_exact(n_rows) {
        bank.extend(ring_switch::fold_1b_rows_split(column, &lo, &hi, &padding));
    }
    bank
}

fn merged_bitbank_optimized() -> bool {
    #[cfg(test)]
    if let Some(enabled) = MERGED_SPLIT_ROWS_OVERRIDE.get() {
        return enabled;
    }
    true
}

/// The alias entry may share banks with ring switching only when every
/// word and bit is part of the same full rectangular address space.
fn full_rectangle_without_padding(
    words: usize,
    heights: &[u64],
    n_log: usize,
    padding: &PaddingSpec,
) -> bool {
    let Some(rows) = 1usize.checked_shl(n_log as u32) else {
        return false;
    };
    if !heights.len().is_power_of_two()
        || heights.iter().any(|&h| h != rows as u64)
        || rows.checked_mul(heights.len()) != Some(words)
    {
        return false;
    }
    let covered = padding.runs().iter().try_fold(0usize, |total, run| {
        let width = 1usize.checked_shl(run.k_log as u32)?;
        if run.useful_bits_per_block != width {
            return None;
        }
        total.checked_add(width.checked_mul(run.n_blocks)?)
    });
    covered.is_some() && covered == words.checked_mul(1 << LOG_PACKING)
}

/// Fold a column bank by a claim's column coordinates. This is precisely
/// its 128-entry s_hat: row and column eq factors multiply independently.
fn column_bank_s_hat(bank: &[F128], column_point: &[F128]) -> Vec<F128> {
    assert_eq!(bank.len(), 1usize << (column_point.len() + LOG_PACKING));
    let mut s_hat = vec![F128::ZERO; 1 << LOG_PACKING];
    for (column, weight) in bank
        .chunks_exact(1 << LOG_PACKING)
        .zip(build_eq_table(column_point))
    {
        if weight.is_zero() {
            continue;
        }
        for (out, &value) in s_hat.iter_mut().zip(column) {
            *out += weight * value;
        }
    }
    s_hat
}

/// Two row points share each input read and bit transpose through the
/// existing native two-way kernel. All tables are still built online.
#[cfg(test)]
fn column_bitbank_split_pair(
    q: &[F128],
    rows: [&[F128]; 2],
    n_log: usize,
    k_cols: usize,
) -> [Vec<F128>; 2] {
    column_bitbank_split_pair_masked(q, rows, n_log, k_cols, [[true; 2]; 2], &[])
}

/// The block-first transport folds the high column bits before the row
/// rounds. Even a zero high-column weight needs its S[e] for cross terms,
/// so a one-hot claim cannot discard all its other columns. Only a zero
/// low-column factor makes the *whole* S polynomial zero. Keep the union
/// of these parity supports for all claims sharing a row point.
fn bitbank_parities(column_point: &[F128]) -> [bool; 2] {
    match column_point.first().copied() {
        None => [true, false],
        Some(z) => [z != F128::ONE, z != F128::ZERO],
    }
}

/// Inspect actual source words once, online, before any row-point folds.
/// Random columns normally fail on their second word; a claimed constant
/// must pass every word, including its last. No circuit metadata or expected
/// padding value substitutes for this check.
fn constant_bitbank_columns(q: &[F128], n_log: usize, k_cols: usize) -> Vec<Option<F128>> {
    use rayon::prelude::{ParallelIterator, ParallelSlice};
    let n_rows = 1usize << n_log;
    let n_cols = 1usize << k_cols;
    assert!(q.len() >= n_rows * n_cols);
    q[..n_rows * n_cols]
        .par_chunks_exact(n_rows)
        .map(|column| {
            let value = column[0];
            column[1..]
                .iter()
                .all(|&word| word == value)
                .then_some(value)
        })
        .collect()
}

/// Ordinary row equality factors sum to one for every challenge, including
/// zero and one. Hence a constant source column's exact 128-bit bank is its
/// bit decomposition. This preserves *all* block cross terms, not merely
/// the contribution at the original column query.
fn constant_column_bitbank(value: F128, out: &mut [F128]) {
    assert_eq!(out.len(), 1 << LOG_PACKING);
    for (bit, slot) in out.iter_mut().enumerate() {
        let limb = if bit < 64 { value.lo } else { value.hi };
        *slot = if (limb >> (bit % 64)) & 1 == 0 {
            F128::ZERO
        } else {
            F128::ONE
        };
    }
}

fn column_bitbank_split_masked(
    q: &[F128],
    row: &[F128],
    n_log: usize,
    k_cols: usize,
    parities: [bool; 2],
    constants: &[Option<F128>],
) -> Vec<F128> {
    if parities == [true; 2] && constants.is_empty() {
        return column_bitbank_split(q, row, n_log, k_cols);
    }
    let n_rows = 1usize << n_log;
    let n_cols = 1usize << k_cols;
    assert_eq!(row.len(), n_log);
    assert!(q.len() >= n_rows * n_cols);
    let mut bank = vec![F128::ZERO; n_cols << LOG_PACKING];
    let split =
        (n_log >= 4).then(|| ring_switch::build_eq_split(row, ring_switch::split_n_lo(n_log)));
    let padding = PaddingSpec::dense(n_log + LOG_PACKING);
    for (c, column) in q[..n_rows * n_cols].chunks_exact(n_rows).enumerate() {
        if !parities[c & 1] {
            continue;
        }
        if let Some(value) = constants.get(c).copied().flatten() {
            constant_column_bitbank(value, &mut bank[c << LOG_PACKING..(c + 1) << LOG_PACKING]);
            continue;
        }
        let values = match &split {
            Some((lo, hi)) => ring_switch::fold_1b_rows_split(column, lo, hi, &padding),
            None => column_bitbank_reference(column, row, n_log, 0),
        };
        bank[c << LOG_PACKING..(c + 1) << LOG_PACKING].copy_from_slice(&values);
    }
    bank
}

fn column_bitbank_split_pair_masked(
    q: &[F128],
    rows: [&[F128]; 2],
    n_log: usize,
    k_cols: usize,
    parities: [[bool; 2]; 2],
    constants: &[Option<F128>],
) -> [Vec<F128>; 2] {
    assert!(rows.iter().all(|r| r.len() == n_log));
    if n_log < 4 {
        return std::array::from_fn(|i| {
            column_bitbank_split_masked(q, rows[i], n_log, k_cols, parities[i], constants)
        });
    }
    let n_rows = 1usize << n_log;
    let n_cols = 1usize << k_cols;
    assert!(q.len() >= n_rows * n_cols);
    let n_lo = ring_switch::split_n_lo(n_log);
    let [(lo0, hi0), (lo1, hi1)] = rows.map(|r| ring_switch::build_eq_split(r, n_lo));
    let padding = PaddingSpec::dense(n_log + LOG_PACKING);
    let mut banks = std::array::from_fn(|_| vec![F128::ZERO; n_cols << LOG_PACKING]);
    for (c, column) in q[..n_rows * n_cols].chunks_exact(n_rows).enumerate() {
        let range = c << LOG_PACKING..(c + 1) << LOG_PACKING;
        if let Some(value) = constants.get(c).copied().flatten() {
            for i in 0..2 {
                if parities[i][c & 1] {
                    constant_column_bitbank(value, &mut banks[i][range.clone()]);
                }
            }
            continue;
        }
        match [parities[0][c & 1], parities[1][c & 1]] {
            [true, true] => {
                let (a, b) =
                    ring_switch::fold_1b_rows_split_2way(column, &lo0, &hi0, &lo1, &hi1, &padding);
                banks[0][range.clone()].copy_from_slice(&a);
                banks[1][range].copy_from_slice(&b);
            }
            [true, false] => banks[0][range].copy_from_slice(&ring_switch::fold_1b_rows_split(
                column, &lo0, &hi0, &padding,
            )),
            [false, true] => banks[1][range].copy_from_slice(&ring_switch::fold_1b_rows_split(
                column, &lo1, &hi1, &padding,
            )),
            [false, false] => {}
        }
    }
    banks
}
/// A bank depends on the row point alone, not the bit weights, column
/// point or batching coefficient of the claim. Keep one bank per exact
/// row point for the duration of this opening; supplied native banks are
/// borrowed. Returned indices retain the original claim order.
fn column_bitbanks_for_claims<'a>(
    q: &[F128],
    x_outers: &[&[F128]],
    supplied: &[Option<&'a [F128]>],
    n_log: usize,
    k_cols: usize,
    trace: bool,
) -> (Vec<std::borrow::Cow<'a, [F128]>>, Vec<usize>) {
    let optimized = merged_bitbank_optimized();
    let sparse = optimized && sparse_bitbanks_enabled();
    let mut points: Vec<&[F128]> = Vec::new();
    let mut parities: Vec<[bool; 2]> = Vec::new();
    let mut banks: Vec<Option<std::borrow::Cow<'a, [F128]>>> = Vec::new();
    let mut indices = Vec::with_capacity(x_outers.len());
    for (i, x) in x_outers.iter().enumerate() {
        let row = &x[1..1 + n_log];
        let needed = if sparse {
            bitbank_parities(&x[1 + n_log..])
        } else {
            [true; 2]
        };
        if optimized && let Some(j) = points.iter().position(|&point| point == row) {
            for c0 in 0..2 {
                parities[j][c0] |= needed[c0];
            }
            indices.push(j);
            continue;
        }
        let bank = match supplied.get(i).copied().flatten() {
            Some(bank) if bank.len() == 1usize << (k_cols + LOG_PACKING) => {
                Some(std::borrow::Cow::Borrowed(bank))
            }
            _ => None,
        };
        indices.push(banks.len());
        points.push(row);
        parities.push(needed);
        banks.push(bank);
    }
    let missing: Vec<usize> = banks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.is_none().then_some(i))
        .collect();
    // Stay off tiny and one/two-column openings: the shared detection pass
    // is intended for nontrivial rectangular banks. Supplied banks need no
    // work and never cause a source scan on their own.
    let inspect_constants = optimized
        && constant_bitbanks_enabled()
        && n_log >= 8
        && k_cols >= 2
        && !missing.is_empty();
    let detection_started = std::time::Instant::now();
    let mut constants = if inspect_constants {
        constant_bitbank_columns(q, n_log, k_cols)
    } else {
        Vec::new()
    };
    let constant_count = constants.iter().filter(|x| x.is_some()).count();
    if trace && inspect_constants {
        eprintln!(
            "  [open_merged] constant source columns: {constant_count}/{} ({:.2} ms online scan)",
            1usize << k_cols,
            detection_started.elapsed().as_secs_f64() * 1e3
        );
    }
    if constant_count == 0 {
        constants.clear();
    }
    let paired = optimized && paired_bitbanks_enabled() && n_log >= 4;
    let mut pairs = 0usize;
    for group in missing.chunks(if paired { 2 } else { 1 }) {
        if let &[a, b] = group {
            let [bank_a, bank_b] = column_bitbank_split_pair_masked(
                q,
                [points[a], points[b]],
                n_log,
                k_cols,
                [parities[a], parities[b]],
                &constants,
            );
            banks[a] = Some(std::borrow::Cow::Owned(bank_a));
            banks[b] = Some(std::borrow::Cow::Owned(bank_b));
            pairs += 1;
        } else {
            let i = group[0];
            banks[i] = Some(std::borrow::Cow::Owned(if optimized {
                column_bitbank_split_masked(q, points[i], n_log, k_cols, parities[i], &constants)
            } else {
                column_bitbank_reference(q, points[i], n_log, k_cols)
            }));
        }
    }
    let banks: Vec<_> = banks
        .into_iter()
        .map(|b| b.expect("every missing bank was filled"))
        .collect();
    if trace {
        let n_cols = 1usize << k_cols;
        let used: usize = parities
            .iter()
            .map(|p| (0..n_cols).filter(|&c| p[c & 1]).count())
            .sum();
        eprintln!(
            "  [open_merged] column bit-bank support: {used}/{} row-point columns (sparse={sparse})",
            banks.len() * n_cols
        );
        eprintln!(
            "  [open_merged] column bit-banks: {} row points for {} claims, {pairs} paired folds (online native fold)",
            banks.len(),
            x_outers.len(),
        );
    }
    (banks, indices)
}
/// The block-first phase of the merged sumcheck (docs, "direct transport").
///
/// The dense index is `d = e·2^(n+1) + c0·2^n + r` (block `e` of two lane
/// columns, column bit `c0`, row `r`). Every weight term is rank-1 across the
/// `(e | c0, r)` split once the fold map is linearized:
/// `W_i[e, (c0, r)] = Σ_j c_ij·u_i(e)^(2^j)·(eq(z_col[0], c0)·eq(z_row, r))^(2^j)`
/// with `u_i(e) = eq(z_col[1..], e)`, masked to the live column prefix. The
/// `k−1` block rounds therefore need only, per term, the block statistics
/// `S_ij[e] = (eq(z_col[0], c0)·Σ_b β_b^(2^-j)·bank[(2e+c0)·128 + b])^(2^j)`
/// — a Frobenius transform of the column bit-bank — against
/// `A_ij[e] = c_ij·eq(z_col[1..]^(2^j), e)`. After them, `q` is folded over
/// the bound block coordinates and the row-side weight `W'` is written in
/// closed form (the block factor folds into the linearized coefficients, the
/// `c0` bit into the byte tables), ready for the ordinary row rounds.
#[allow(clippy::too_many_arguments)]
fn block_first_phase<Ch: Challenger>(
    q: &[F128],
    n_log: usize,
    k_cols: usize,
    live_cols: usize,
    rs: &[BlockFirstClaim<'_>],
    pd_groups: &[(&[F128], Vec<F128>)],
    grinding: OpeningGrinding,
    challenger: &mut Ch,
    merged_rounds: &mut Vec<(F128, F128)>,
    merged_round_nonces: &mut Vec<u64>,
    r_blk: &mut Vec<F128>,
    trace: bool,
) -> (Vec<F128>, Vec<F128>) {
    use rayon::prelude::{
        IndexedParallelIterator, IntoParallelIterator, ParallelIterator, ParallelSliceMut,
    };
    const NB: usize = 1 << LOG_PACKING;
    let t = std::time::Instant::now();
    let kb = k_cols - 1;
    let n_e = 1usize << kb;
    let blk = 1usize << (n_log + 1);
    let n_rows = 1usize << n_log;
    // Live blocks per column bit: column `2e + c0` is live iff `2e + c0 < p`.
    let live_e = |c0: usize| (live_cols + 1 - c0) / 2;
    // Frobenius powers of the bit basis: `pw[b][k] = β_b^(2^k)`.
    let pw: Vec<Vec<F128>> = (0..NB)
        .map(|b| {
            let mut row = vec![F128::ZERO; NB];
            row[0] = ring_switch::bit_basis(b);
            for k in 1..NB {
                row[k] = row[k - 1].square();
            }
            row
        })
        .collect();

    // Rank-1 terms on the block cube.
    let mut terms: Vec<(Vec<F128>, Vec<F128>)> = Vec::new();
    let sparse_columns = sparse_bitbanks_enabled();
    for cl in rs {
        let z0 = cl.z_col[0];
        let z_hi = &cl.z_col[1..];
        let per_j: Vec<(Vec<F128>, Vec<F128>)> = (0..NB)
            .into_par_iter()
            .filter(|&j| cl.coeffs[j] != F128::ZERO)
            .flat_map_iter(|j| {
                let pw = &pw;
                let zj: Vec<F128> = z_hi.iter().map(|&z| frob(z, j)).collect();
                let eq_hi = crate::lincheck::build_eq_table(&zj);
                let kinv = (NB - j) % NB;
                (0..2)
                    .filter(move |&c0| {
                        // A zero low-column factor makes the complete S[e]
                        // polynomial zero, before and after every block fold.
                        !sparse_columns || if c0 == 0 { F128::ONE + z0 } else { z0 } != F128::ZERO
                    })
                    .map(move |c0| {
                        let kappa = if c0 == 0 { F128::ONE + z0 } else { z0 };
                        let live = live_e(c0);
                        let a: Vec<F128> = (0..n_e)
                            .map(|e| {
                                if e < live {
                                    cl.coeffs[j] * eq_hi[e]
                                } else {
                                    F128::ZERO
                                }
                            })
                            .collect();
                        let s: Vec<F128> = (0..n_e)
                            .map(|e| {
                                let bank = &cl.bank[(2 * e + c0) * NB..(2 * e + c0 + 1) * NB];
                                let mut acc = F128::ZERO;
                                for b in 0..NB {
                                    acc += pw[b][kinv] * bank[b];
                                }
                                frob(kappa * acc, j)
                            })
                            .collect();
                        (a, s)
                    })
            })
            .collect();
        terms.extend(per_j);
    }
    for (z_row, cols) in pd_groups {
        let eq_row = MergedRowEq::new(z_row);
        let f = direct_block_statistics(q, n_log, k_cols, live_cols, &eq_row);
        for c0 in 0..2 {
            let live = live_e(c0);
            let a: Vec<F128> = (0..n_e)
                .map(|e| {
                    if e < live {
                        cols[2 * e + c0]
                    } else {
                        F128::ZERO
                    }
                })
                .collect();
            let s: Vec<F128> = (0..n_e).map(|e| f[2 * e + c0]).collect();
            terms.push((a, s));
        }
    }
    if trace {
        eprintln!(
            "  [open_merged] block statistics ({} terms × {n_e}): {:6.2} ms",
            terms.len(),
            t.elapsed().as_secs_f64() * 1e3
        );
    }

    // The block rounds.
    let t = std::time::Instant::now();
    let (mut g_one, mut g_inf) = round_message_terms(&terms);
    for round in 0..kb {
        challenger.observe_f128(g_one);
        challenger.observe_f128(g_inf);
        let r = if grinding.merged_round_bits != 0 {
            let (nonce, r) = challenger.grind_pow_and_sample_f128(grinding.merged_round_bits);
            merged_round_nonces.push(nonce);
            r
        } else {
            challenger.sample_f128()
        };
        merged_rounds.push((g_one, g_inf));
        r_blk.push(r);
        for (a, s) in terms.iter_mut() {
            fold_pairs_in_place(a, r);
            fold_pairs_in_place(s, r);
        }
        if round + 1 < kb {
            (g_one, g_inf) = round_message_terms(&terms);
        }
    }

    if trace {
        eprintln!(
            "  [open_merged] block rounds only ({kb}): {:6.2} ms",
            t.elapsed().as_secs_f64() * 1e3
        );
    }
    let t_qp = trace.then(std::time::Instant::now);
    // q' = q folded over the bound block coordinates.
    let eq_blk = crate::lincheck::build_eq_table(r_blk);
    let n_live_e = live_cols.div_ceil(2);
    let mut qp = vec![F128::ZERO; blk];
    qp.par_chunks_mut(2048).enumerate().for_each(|(ci, chunk)| {
        let off = ci * 2048;
        for e in 0..n_live_e {
            let w = eq_blk[e];
            let src = &q[e * blk + off..e * blk + off + chunk.len()];
            for (d, &x) in chunk.iter_mut().zip(src) {
                *d += w * x;
            }
        }
    });

    if let Some(t_qp) = t_qp {
        eprintln!(
            "  [open_merged] q-prime fold: {:6.2} ms",
            t_qp.elapsed().as_secs_f64() * 1e3
        );
    }
    let t_wp_setup = trace.then(std::time::Instant::now);
    // W'[(c0, r)] = Σ_{e live for c0} eq(r_blk, e)·W[e, (c0, r)], in closed
    // form: the block factor folds into the linearized coefficients,
    // c'_{j,c0} = c_j·Σ_e eq(r_blk, e)·eq(z_col[1..]^(2^j), e), and the
    // column bit bakes eq(z_col[0], c0) into the per-c0 byte tables.
    // For a fixed row point E(r), sum_i Phi_i(E(r)) equals
    // (sum_i Phi_i)(E(r)). Accumulate the two column-bit maps in their
    // 128 linearized coefficients before expanding to byte tables. This
    // does not combine original claims or alter the preceding block rounds:
    // it only avoids applying one map per claim at every output row.
    let group_rows = merged_row_grouping_enabled();
    let mut row_points: Vec<&[F128]> = Vec::new();
    let mut row_maps: Vec<[Vec<F128>; 2]> = Vec::new();
    for cl in rs {
        let hot: Option<usize> = if group_rows {
            cl.z_col
                .iter()
                .enumerate()
                .try_fold(0, |column, (bit, &value)| {
                    if value == F128::ZERO {
                        Some(column)
                    } else if value == F128::ONE {
                        Some(column | (1 << bit))
                    } else {
                        None
                    }
                })
        } else {
            None
        };
        let maps = if let Some(column) = hot {
            // Boolean column coordinates remain unchanged under Frobenius.
            // Every conjugate eq table is the same one-hot indicator, so
            // its block contraction is a single public-index lookup.
            [0usize, 1].map(|c0| {
                let weight = if column < live_cols && column & 1 == c0 {
                    eq_blk[column / 2]
                } else {
                    F128::ZERO
                };
                cl.coeffs.iter().map(|&c| c * weight).collect()
            })
        } else {
            let z0 = cl.z_col[0];
            let z_hi = &cl.z_col[1..];
            let eq_hi_j: Vec<Vec<F128>> = (0..NB)
                .map(|j| {
                    let zj: Vec<F128> = z_hi.iter().map(|&z| frob(z, j)).collect();
                    crate::lincheck::build_eq_table(&zj)
                })
                .collect();
            [0usize, 1].map(|c0| {
                let kappa = if c0 == 0 { F128::ONE + z0 } else { z0 };
                let live = live_e(c0);
                (0..NB)
                    .map(|j| {
                        let s =
                            (0..live).fold(F128::ZERO, |acc, e| acc + eq_blk[e] * eq_hi_j[j][e]);
                        cl.coeffs[j] * s * frob(kappa, j)
                    })
                    .collect::<Vec<F128>>()
            })
        };
        if group_rows && let Some(g) = row_points.iter().position(|&p| p == cl.z_row) {
            for (sum, map) in row_maps[g].iter_mut().zip(maps) {
                for (dst, value) in sum.iter_mut().zip(map) {
                    *dst += value;
                }
            }
        } else {
            row_points.push(cl.z_row);
            row_maps.push(maps);
        }
    }
    let eq_rows: Vec<MergedRowEq> = row_points.iter().map(|p| MergedRowEq::new(p)).collect();
    let tabs: Vec<[Vec<F128>; 2]> = row_maps
        .iter()
        .map(|maps| {
            [0usize, 1].map(|c0| {
                let cp = &maps[c0];
                let w: Vec<F128> = (0..NB)
                    .map(|b| (0..NB).fold(F128::ZERO, |acc, j| acc + cp[j] * pw[b][j]))
                    .collect();
                ring_switch::build_fold_byte_table(&w)
            })
        })
        .collect();
    if trace {
        eprintln!(
            "  [open_merged] rectangular row maps: {} claims -> {} row points (grouping={group_rows})",
            rs.len(),
            row_points.len()
        );
    }
    let pd: Vec<(MergedRowEq, [F128; 2])> = pd_groups
        .iter()
        .map(|(z_row, cols)| {
            let eq_row = MergedRowEq::new(z_row);
            let s = [0usize, 1].map(|c0| {
                (0..live_e(c0)).fold(F128::ZERO, |acc, e| acc + eq_blk[e] * cols[2 * e + c0])
            });
            (eq_row, s)
        })
        .collect();
    let direct_wp_simd = direct_wp_simd_enabled() && !pd.is_empty();
    let scalar_pd = if direct_wp_simd {
        &[][..]
    } else {
        pd.as_slice()
    };
    if let Some(t_wp_setup) = t_wp_setup {
        eprintln!(
            "  [open_merged] W-prime setup: {:6.2} ms",
            t_wp_setup.elapsed().as_secs_f64() * 1e3
        );
    }
    let t_wp = trace.then(std::time::Instant::now);
    let wp_uninit = wp_uninit_enabled();
    let t_wp_allocate = trace.then(std::time::Instant::now);
    let mut wp = if wp_uninit {
        Vec::<F128>::with_capacity(blk)
    } else {
        vec![F128::ZERO; blk]
    };
    if let Some(t_wp_allocate) = t_wp_allocate {
        eprintln!(
            "  [open_merged] W-prime allocate/zero-init (uninit={wp_uninit}): {:6.2} ms",
            t_wp_allocate.elapsed().as_secs_f64() * 1e3
        );
    }
    let t_wp_fill = trace.then(std::time::Instant::now);
    if wp_uninit {
        wp.spare_capacity_mut()[..blk]
            .par_chunks_mut(2048)
            .enumerate()
            .for_each(|(ci, chunk)| {
                initialize_wp_chunk(
                    chunk,
                    ci * 2048,
                    n_log,
                    &eq_rows,
                    &tabs,
                    &pd,
                    direct_wp_simd,
                );
            });
        // SAFETY: Rayon has joined all disjoint workers, each of which wrote
        // its entire chunk before returning. On panic set_len is skipped and
        // the still-empty Vec safely frees its allocation during unwinding.
        unsafe { wp.set_len(blk) };
    } else {
        // Keep the initialized reference path for an isolated allocation
        // ablation, including its existing row arithmetic and direct order.
        wp.par_chunks_mut(2048).enumerate().for_each(|(ci, chunk)| {
            let off = ci * 2048;
            for (i, slot) in chunk.iter_mut().enumerate() {
                let h = off + i;
                let c0 = h >> n_log;
                let r = h & (n_rows - 1);
                let mut acc = F128::ZERO;
                for (eq_row, tab) in eq_rows.iter().zip(&tabs) {
                    acc += ring_switch::fold_one_slot(eq_row.at(r), &tab[c0]);
                }
                for (eq_row, s) in scalar_pd {
                    acc += s[c0] * eq_row.at(r);
                }
                *slot = acc;
            }
            if direct_wp_simd {
                direct_wp_add_groups(chunk, off, n_log, &pd);
            }
        });
    }
    if let Some(t_wp_fill) = t_wp_fill {
        eprintln!(
            "  [open_merged] W-prime fill (uninit={wp_uninit}): {:6.2} ms",
            t_wp_fill.elapsed().as_secs_f64() * 1e3
        );
    }
    if let Some(t_wp) = t_wp {
        eprintln!(
            "  [open_merged] W-prime generation (direct_simd={direct_wp_simd}, {} direct groups): {:6.2} ms",
            pd.len(),
            t_wp.elapsed().as_secs_f64() * 1e3
        );
    }
    if trace {
        eprintln!(
            "  [open_merged] block rounds ({kb}) + q'/W' (2^{}): {:6.2} ms",
            n_log + 1,
            t.elapsed().as_secs_f64() * 1e3
        );
    }
    (qp, wp)
}

/// The remaining rounds of the merged product sumcheck over `(a0, b0)`
/// (LSB-first pairs), starting from the caller's round message. `live` bounds
/// the non-zero prefix of `a0` (fold ranges shrink with it; `b0` must be zero
/// on `[area, live)`). Appends messages, grinding nonces and challenges;
/// returns the fully folded pair and the statistics seed captured at
/// `seed_len` (see `open_batch_mixed_ligerito_seeded`).
#[allow(clippy::too_many_arguments)]
fn merged_sumcheck_rounds<Ch: Challenger>(
    a0: &[F128],
    b0: &[F128],
    live: usize,
    mut g_one: F128,
    mut g_inf: F128,
    seed_len: usize,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
    merged_rounds: &mut Vec<(F128, F128)>,
    merged_round_nonces: &mut Vec<u64>,
    rho: &mut Vec<F128>,
) -> (F128, F128, Option<Vec<F128>>) {
    let l = a0.len();
    debug_assert_eq!(b0.len(), l);
    let n_rounds = l.trailing_zeros() as usize;
    debug_assert_eq!(1usize << n_rounds, l);
    if n_rounds == 0 {
        return (a0[0], b0[0], None);
    }
    let mut live = live.min(l);
    let mut sa = crate::scratch::take_f128(l / 2);
    let mut sb = crate::scratch::take_f128(l / 2);
    let mut a = crate::scratch::take_f128(l / 4);
    let mut bb = crate::scratch::take_f128(l / 4);
    let mut cur = l;
    let mut seeded_stats: Option<Vec<F128>> = None;
    for round in 0..n_rounds {
        let half = cur / 2;
        challenger.observe_f128(g_one);
        challenger.observe_f128(g_inf);
        let r = if grinding.merged_round_bits != 0 {
            let (nonce, r) = challenger.grind_pow_and_sample_f128(grinding.merged_round_bits);
            merged_round_nonces.push(nonce);
            r
        } else {
            challenger.sample_f128()
        };
        merged_rounds.push((g_one, g_inf));
        rho.push(r);
        let (a_src, b_src): (&[F128], &[F128]) = if round == 0 { (a0, b0) } else { (&a, &bb) };
        if cur == seed_len && seed_len < l {
            let lv = live.min(cur);
            let mut g = a_src[..lv].to_vec();
            g.resize(cur, F128::ZERO);
            seeded_stats = Some(g);
        }
        if cur > 2 {
            let lv = live.min(cur);
            let lhalf = lv / 2;
            (g_one, g_inf) = jagged::fold_and_round_oop_par(
                &a_src[..lv],
                &b_src[..lv],
                r,
                &mut sa[..lhalf],
                &mut sb[..lhalf],
            );
            let next = lhalf.next_multiple_of(4).min(half).max(lhalf);
            for i in lhalf..next {
                sa[i] = F128::ZERO;
                sb[i] = F128::ZERO;
            }
            live = next;
        } else {
            jagged::fold_oop_par(
                &a_src[..cur],
                &b_src[..cur],
                r,
                &mut sa[..half],
                &mut sb[..half],
            );
        }
        std::mem::swap(&mut a, &mut sa);
        std::mem::swap(&mut bb, &mut sb);
        cur = half;
    }
    let out = (a[0], bb[0], seeded_stats);
    crate::scratch::give_f128(sa);
    crate::scratch::give_f128(sb);
    crate::scratch::give_f128(a);
    crate::scratch::give_f128(bb);
    out
}

#[allow(clippy::too_many_arguments)]
pub fn open_batch_merged<Ch: Challenger>(
    q: Vec<F128>,
    // The M-variable padded buffer the ring switch reads. `None` means it IS
    // `q` — identity compaction, where the caller moved one buffer in rather
    // than cloning it to satisfy the by-value/by-ref split.
    padded_witness: Option<&[F128]>,
    prover_data: &ProverData,
    commitment: &Commitment,
    // The first coordinate belongs to the legacy skip64 claim check and is
    // not read by this prover: the row/column tensor starts at x_outer[1..].
    // A caller using verify_batch_merged_weighted may put a dummy coordinate
    // first, and supply its complete 128 bit-column weights to the verifier.
    x_outers: &[&[F128]],
    precomputed_s_hat_v: &[Option<&[F128]>],
    // Per ring-switch claim, the column bit-bank at its row point (see
    // `lincheck::union_bitbank_fold`); consumed by the block-first transport
    // when the stack is a full-height column prefix. Missing entries are
    // rebuilt online from `q` with the native split-tensor bit fold.
    column_bitbanks: &[Option<&[F128]>],
    packed_direct: &[PackedDirectClaim],
    padding: &PaddingSpec,
    heights: &[u64],
    n_log: usize,
    lig_config: &ProverConfig,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
) -> MergedOpenProof {
    let trace = var("PCS_TRACE").is_ok();
    let t_total = Instant::now();
    // Belt-and-braces on the cap-depth derivation: the commit-time cap
    // (from `PcsParams::level_zero_cap_depth`) must be the layer the opener's
    // config implies — a config-source disagreement fails loudly here at
    // prove time instead of as a verifier reject.
    assert_eq!(
        commitment.cap.len(),
        1usize << lig_config.level_zero_cap_depth(),
        "commitment cap size disagrees with the opener config's L0 query count"
    );
    debug_assert_eq!(
        commitment.cap.as_slice(),
        cap_layer(
            &prover_data.merkle_tree,
            commitment.params.n_leaves(),
            lig_config.level_zero_cap_depth(),
        ),
        "commitment cap is not the prover tree's cap layer"
    );
    challenger.observe_label(b"flock-merged-open-v1");
    let t = Instant::now();
    // Element-only registries produce no ring-switched claims; skip the batch
    // entirely (the callee asserts a non-empty batch). This branch DEFINES the
    // element-only merged transcript: nothing is absorbed for the empty batch,
    // exactly as in the mixed open's `n_rs > 0` guard.
    // Full identity aliases can share one row-bank sweep between the ring
    // switch and block-first transport. No transcript challenge is moved:
    // the bank and its 128-entry column fold depend only on existing query
    // points. Other padded/compacted layouts retain the existing route.
    let mut early_banks = if padded_witness.is_none()
        && merged_bitbank_optimized()
        && !x_outers.is_empty()
        && full_rectangle_without_padding(q.len(), heights, n_log, padding)
    {
        Some(column_bitbanks_for_claims(
            &q,
            x_outers,
            column_bitbanks,
            n_log,
            heights.len().ilog2() as usize,
            trace,
        ))
    } else {
        None
    };
    let calculated_s_hats: Vec<Option<Vec<F128>>> = match early_banks.as_ref() {
        Some((banks, indices)) => {
            assert!(
                precomputed_s_hat_v.is_empty() || precomputed_s_hat_v.len() == x_outers.len(),
                "precomputed_s_hat_v: must be empty or match claim count"
            );
            x_outers
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    if precomputed_s_hat_v.get(i).copied().flatten().is_some() {
                        None
                    } else {
                        Some(column_bank_s_hat(&banks[indices[i]], &point[1 + n_log..]))
                    }
                })
                .collect()
        }
        None => Vec::new(),
    };
    let rs_precomputed: Vec<Option<&[F128]>> = calculated_s_hats
        .iter()
        .enumerate()
        .map(|(i, value)| {
            precomputed_s_hat_v
                .get(i)
                .copied()
                .flatten()
                .or(value.as_deref())
        })
        .collect();
    let padded_witness = padded_witness.unwrap_or(&q);
    let batch_bits = grinding.claim_batch_bits_for(x_outers.len() + packed_direct.len());
    let mut rs_results = if !x_outers.is_empty() {
        prove_batched_padded_with_precomputed_unbatched_and_grinding(
            padded_witness,
            x_outers,
            if early_banks.is_some() {
                &rs_precomputed
            } else {
                precomputed_s_hat_v
            },
            padding,
            grinding.ring_switch_bits,
            challenger,
        )
    } else {
        Vec::new()
    };
    if trace {
        eprintln!(
            "  [open_merged] ring_switch: {:6.2} ms",
            t.elapsed().as_secs_f64() * 1e3
        );
    }
    // Packed-direct claims take their γ's from the same stream, AFTER the
    // ring-switched ones — the verifier draws them in the same order.
    //
    // VALUE-ONLY absorb (v1): a claim's POINT is never the prover's to choose
    // — every packed-direct caller derives it from earlier transcript
    // challenges plus statement constants (gather points from the wiring
    // GKR's ρ and the cell space, element points from the region PIOP's own
    // challenges and the frozen prefix), and the verifier RECOMPUTES it
    // rather than reading it from the proof. Absorbing a deterministic
    // public function of (statement, transcript-prefix) adds no binding, and
    // at ~24 words × hundreds of claims it was ~92 KB of transcript the
    // recursion circuit re-hashed per child. The γ's still bind the VALUES,
    // which are genuine prover messages. (The non-merged `open_batch` has
    // absorbed value-only since birth — this aligns the merged intake.)
    for c in packed_direct {
        challenger.observe_f128(c.value);
    }
    let mut batching_nonces = Vec::with_capacity(usize::from(batch_bits != 0));
    let n_gammas = x_outers.len() + packed_direct.len();
    let gammas = if batch_bits != 0 {
        let (nonce, gammas) = challenger.grind_pow_and_sample_f128_vec(batch_bits, n_gammas);
        batching_nonces.push(nonce);
        gammas
    } else {
        challenger.sample_f128_vec(n_gammas)
    };
    let (gammas_rs, gammas_pd) = gammas.split_at(x_outers.len());
    for ((_, output), &gamma) in rs_results.iter_mut().zip(gammas_rs) {
        output.rs_eq_ind.scale_in_place(gamma);
    }

    let mut target = F128::ZERO;
    for ((_, o), g) in rs_results.iter().zip(gammas_rs.iter()) {
        target += *g * o.sumcheck_claim;
    }
    for (c, g) in packed_direct.iter().zip(gammas_pd.iter()) {
        target += *g * c.value;
    }

    let dense_log = commitment.params.m - LOG_PACKING;
    assert_eq!(
        q.len(),
        1usize << dense_log,
        "q must be the committed stack"
    );
    let params = JaggedParams::from_heights(heights, n_log, dense_log);
    let k_cols = params.k;
    // Packed-direct claims are γ-SCALAR maps (`x ↦ γ·x`), so claims sharing
    // a row point collapse into merged-column scalar groups, built ONCE and
    // consumed by the weight builder, the multipoint protocol, and the
    // anchor ([`scalar_claim_groups`]). A packed-direct point carries NO
    // univariate-skip coordinate, which is the one place the splits differ.
    let pd_groups = scalar_claim_groups(
        packed_direct.iter().map(|c| c.point.as_slice()),
        gammas_pd,
        n_log,
        k_cols,
    );
    // Ring-switched claims enter the weight builder with their F₂-linear
    // fold tables; the scalar groups with their precombined column tables —
    // `Σᵢ γᵢ·eq_colᵢ` — instead of a per-claim fold-table sweep
    // (bit-identical W; see `MergedWeightClaim::Scalar`). The circuit
    // path's gather claims all share ρ_row, so its ~2^c claims cost one
    // sweep.
    let mut weight_claims: Vec<MergedWeightClaim<'_>> = rs_results
        .iter()
        .zip(x_outers.iter())
        .map(|((_, o), x)| {
            assert_eq!(x.len(), 1 + n_log + k_cols, "point/row/col split mismatch");
            let table = match &o.rs_eq_ind {
                RsEqInd::DeferredDense { table, .. } => table.as_slice(),
                _ => panic!("merged open requires DeferredDense ring-switch claims"),
            };
            MergedWeightClaim::Folded {
                z_row: &x[1..1 + n_log],
                z_col: &x[1 + n_log..],
                table,
            }
        })
        .collect();
    weight_claims.extend(
        pd_groups
            .iter()
            .map(|(z_row, cols)| MergedWeightClaim::Scalar { z_row, cols }),
    );

    // Linearized coefficients of the (γ-scaled) fold tables: the block-first
    // transport needs them now; the Frobenius assist below reuses them.
    let coeffs: Vec<Vec<F128>> = rs_results
        .iter()
        .map(|(_, o)| match &o.rs_eq_ind {
            RsEqInd::DeferredDense { table, .. } => linearized_coefficients(table),
            _ => unreachable!("checked above"),
        })
        .collect();

    let t = std::time::Instant::now();
    let mut merged_rounds = Vec::with_capacity(dense_log);
    let mut merged_round_nonces =
        Vec::with_capacity((grinding.merged_round_bits != 0) as usize * dense_log);
    let mut rho = Vec::with_capacity(dense_log);
    let seed_len = 1usize << lig_config.initial_k;
    let (q_eval, w_eval, seeded_stats) = if let Some(live_cols) =
        jagged::rectangular_prefix_columns(heights, n_log)
    {
        // Block-first transport (docs, "direct transport"): a full-height
        // column prefix lets the lane-block coordinates be bound first
        // from O(2^k) block statistics — no 2^m weight vector — and the
        // row rounds then run on the block-folded pair. The verifier
        // rotates ρ back into coordinate order under the same predicate.
        let (banks, bank_indices) = early_banks.take().unwrap_or_else(|| {
            column_bitbanks_for_claims(&q, x_outers, column_bitbanks, n_log, k_cols, trace)
        });
        let rs: Vec<BlockFirstClaim<'_>> = x_outers
            .iter()
            .zip(&coeffs)
            .zip(&bank_indices)
            .map(|((x, c), &bank)| BlockFirstClaim {
                z_row: &x[1..1 + n_log],
                z_col: &x[1 + n_log..],
                coeffs: c,
                bank: &banks[bank],
            })
            .collect();
        let (qp, wp) = block_first_phase(
            &q,
            n_log,
            k_cols,
            live_cols,
            &rs,
            &pd_groups,
            grinding,
            challenger,
            &mut merged_rounds,
            &mut merged_round_nonces,
            &mut rho,
            trace,
        );
        let (g_one, g_inf) = round_message_pairs(&qp, &wp);
        let (q_eval, w_eval, _) = merged_sumcheck_rounds(
            &qp,
            &wp,
            qp.len(),
            g_one,
            g_inf,
            usize::MAX,
            grinding,
            challenger,
            &mut merged_rounds,
            &mut merged_round_nonces,
            &mut rho,
        );
        // Round order → coordinate order: the first k−1 rounds bound the
        // block coordinates n+1.. of the dense index.
        rho.rotate_left(k_cols - 1);
        (q_eval, w_eval, None)
    } else {
        // The twisted weight over the dense cube (count-proportional
        // Φ-pass; zero tail past the jagged area).
        let (mut w, (u0, u2)) = jagged::build_merged_weight_and_prime(&params, &weight_claims, &q);
        if trace {
            eprintln!(
                "  [open_merged] W build + round-0 prime (2^{} words): {:6.2} ms",
                dense_log,
                t.elapsed().as_secs_f64() * 1e3
            );
        }
        let l = q.len();
        let area = (params.area() as usize).min(l);
        let live = area.next_multiple_of(4).clamp(4, l);
        for slot in &mut w[area..live] {
            *slot = F128::ZERO;
        }
        let out = merged_sumcheck_rounds(
            &q,
            &w,
            live,
            target + u0,
            u2,
            seed_len,
            grinding,
            challenger,
            &mut merged_rounds,
            &mut merged_round_nonces,
            &mut rho,
        );
        crate::scratch::give_f128(w);
        out
    };
    if trace {
        eprintln!(
            "  [open_merged] merged sumcheck ({dense_log} rounds): {:6.2} ms",
            t.elapsed().as_secs_f64() * 1e3
        );
    }

    // The merged transport's four ping-pong buffers are idle now. At m39
    // with four full columns they retain 48 GiB; the interleaved Ligerito
    // first fold needs two fresh 64 GiB F256 buffers while q is still live.
    // Release idle scratch before that allocation, retaining the normal
    // pooled behavior for smaller proofs. No live buffer or transcript
    // state is modified, and both branches of the following join start
    // only after this phase boundary.
    let release_transport_scratch = commitment.params.m >= 36;
    #[cfg(test)]
    let release_transport_scratch = MERGED_SPLIT_ROWS_OVERRIDE
        .get()
        .unwrap_or(release_transport_scratch);
    if release_transport_scratch {
        crate::scratch::clear();
    }

    // ---- Frobenius assist: proves V = Ŵ(ρ).
    let t = std::time::Instant::now();
    let fclaims: Vec<jagged::FrobeniusClaim<'_>> = x_outers
        .iter()
        .zip(&coeffs)
        .map(|(x, c)| FrobeniusClaim {
            z_row: &x[1..1 + n_log],
            z_col: &x[1 + n_log..],
            coeffs: c,
        })
        .collect();
    // The packed-direct claims enter as the merged-column scalar groups:
    // one untwisted dual value per group instead of 128 per claim.
    let gclaims: Vec<ScalarGroupClaim<'_>> = pd_groups
        .iter()
        .map(|(z_row, cols)| ScalarGroupClaim { z_row, cols })
        .collect();
    if trace {
        eprintln!(
            "    [frobenius] linearized_coefficients (x{}): {:6.2} ms",
            coeffs.len(),
            t.elapsed().as_secs_f64() * 1e3
        );
    }
    let t_assist = Instant::now();
    // ---- eq-basis Ligerito opening of q̂(ρ): one packed-direct claim on
    // the existing mixed path (whose verifier evaluates eq residuals in
    // closed form — no b_tilde machinery).
    let pd = PackedDirectClaim {
        point: rho.clone(),
        value: q_eval,
        eq_ind: DirectEqInd::EqPoint(rho.clone()),
    };
    // FORK/JOIN: the assist and the inner open are SIBLINGS, not a chain.
    // Both consume only `rho` and `q_eval` from the merged sumcheck above,
    // and neither reads the other's output — the assist's `V` is checked
    // against the sumcheck's folded claim, not against the opening. So they
    // run CONCURRENTLY on domain-separated chains: the assist takes a child
    // seeded from the parent, the inner open continues on the parent, and
    // the child's closing digest merges after. Sound for the same reason
    // the wiring fork is: both branches bind only the prefix through ρ,
    // which the fork point already covers.
    //
    // The merge is what binds the child's chain into the parent even though
    // nothing samples after it here — keep it, so composing this open into
    // a longer transcript stays sound without revisiting the fork.
    let mut ch_a = challenger.fork(b"flock-par-assist-v1");
    // The two-product multipoint replacement
    // (docs/multipoint-twisted-assist.tex): 128R + P claimed dual values +
    // one two-product sumcheck + ONE untwisted anchor, instead of a
    // per-statement assist — family K collapses, and every verifier piece
    // is a shape the recursion circuit already has.
    let (frobenius, inner) = join(
        || {
            prove_multipoint_twisted_with_grinding(
                &params,
                &fclaims,
                &gclaims,
                &rho,
                grinding.multipoint,
                &mut ch_a,
            )
        },
        || {
            open_batch_mixed_ligerito_seeded(
                q,
                prover_data,
                commitment,
                &[],
                &[],
                &[pd],
                &PaddingSpec::dense(commitment.params.m),
                lig_config,
                grinding,
                seeded_stats,
                challenger,
            )
        },
    );
    challenger.merge_child(ch_a);
    if trace {
        eprintln!(
            "  [open_merged] coeffs + (multipoint assist ∥ inner open): {:6.2} ms \
             (join wall {:6.2} ms = max of the two branches)",
            t.elapsed().as_secs_f64() * 1e3,
            t_assist.elapsed().as_secs_f64() * 1e3
        );
    }
    #[cfg(debug_assertions)]
    {
        let mut v = F128::ZERO;
        for (cl, vs) in fclaims.iter().zip(&frobenius.values) {
            for (j, &a) in vs.iter().enumerate() {
                let mut t = a;
                for _ in 0..j {
                    t *= t;
                }
                v += cl.coeffs[j] * t;
            }
        }
        for &b in &frobenius.group_values {
            v += b;
        }
        debug_assert_eq!(v, w_eval, "multipoint V must equal the folded weight MLE");
    }
    let _ = w_eval;

    if trace {
        eprintln!(
            "  [open_merged] TOTAL: {:6.2} ms",
            t_total.elapsed().as_secs_f64() * 1e3
        );
    }
    drop(fclaims);
    MergedOpenProof {
        ring_switches: rs_results.into_iter().map(|(p, _)| p).collect(),
        batching_nonces,
        merged_rounds,
        merged_round_nonces,
        q_eval,
        frobenius,
        inner,
    }
}

#[allow(clippy::too_many_arguments)]
/// Rebuild the member decomposition [`scalar_claim_groups`] packed, express
/// every count-dependent `W`-value as a RAW claim on the layout table, and
/// tie the decomposition to the anchor's own statement factors — exactly,
/// since bilinear forms are linear in their weights and `GF(2^128)` sums
/// reassociate. Panics on any mismatch: an export that does not recombine
/// to what the verifier itself checked must never leave the process.
fn assemble_jagged_assertion(
    params: &JaggedParams,
    x_outers: &[&[F128]],
    packed_direct: &[PackedDirectClaimRef<'_>],
    gammas_pd: &[F128],
    pd_groups: &[(&[F128], Vec<F128>)],
    n_log: usize,
    mp: &MultipointDefer,
) -> JaggedAssertion {
    let table = JaggedTable::from_params(params);

    // RS claims: raw eq(z_col) at σ, claim order.
    let rs: Vec<JaggedClaim> = x_outers
        .iter()
        .map(|x| {
            JaggedClaim::honest(
                JaggedRowWeight::eq(x[1 + n_log..].to_vec()),
                mp.sigma.clone(),
                &table,
            )
        })
        .collect();

    // The pd members re-grouped by row point in scalar_claim_groups' exact
    // order: boolean columns are one-hot addresses, everything else stays a
    // dense member. The regrouping must mirror the groups the anchor saw.
    struct G<'a> {
        z_row: &'a [F128],
        combo: Vec<(F128, u32)>,
        dense: Vec<(F128, &'a [F128])>,
    }
    let mut gs: Vec<G<'_>> = Vec::new();
    for (c, &g) in packed_direct.iter().zip(gammas_pd) {
        let (zr, zc) = (&c.point[..n_log], &c.point[n_log..]);
        let hot: Option<usize> = zc.iter().enumerate().try_fold(0usize, |acc, (i, &x)| {
            if x == F128::ZERO {
                Some(acc)
            } else if x == F128::ONE {
                Some(acc | (1 << i))
            } else {
                None
            }
        });
        let slot = match gs.iter_mut().find(|m| m.z_row == zr) {
            Some(m) => m,
            None => {
                gs.push(G {
                    z_row: zr,
                    combo: Vec::new(),
                    dense: Vec::new(),
                });
                gs.last_mut().unwrap()
            }
        };
        match hot {
            Some(h) => slot.combo.push((g, h as u32)),
            None => slot.dense.push((g, zc)),
        }
    }
    assert_eq!(
        gs.len(),
        pd_groups.len(),
        "the regrouping must mirror scalar_claim_groups"
    );
    for (m, (zr, _)) in gs.iter().zip(pd_groups) {
        assert_eq!(m.z_row, *zr, "group order must mirror scalar_claim_groups");
    }

    let groups: Vec<(Option<JaggedClaim>, Vec<(F128, JaggedClaim)>)> = gs
        .iter()
        .map(|m| {
            let combo = (!m.combo.is_empty()).then(|| {
                JaggedClaim::honest(
                    JaggedRowWeight::Combo(m.combo.clone()),
                    mp.sigma.clone(),
                    &table,
                )
            });
            let dense = m
                .dense
                .iter()
                .map(|&(g, zc)| {
                    (
                        g,
                        JaggedClaim::honest(
                            JaggedRowWeight::eq(zc.to_vec()),
                            mp.sigma.clone(),
                            &table,
                        ),
                    )
                })
                .collect();
            (combo, dense)
        })
        .collect();

    // THE TIE: the decomposition recombines to the anchor's own statement
    // factors. Statement order is frobenius_statements' — RS claims with a
    // live coefficient first, then the groups. A row-point collision would
    // merge statements and break the mapping, so its absence is asserted.
    for (i, xa) in x_outers.iter().enumerate() {
        for xb in &x_outers[i + 1..] {
            assert!(
                xa[1..1 + n_log] != xb[1..1 + n_log],
                "RS row points must be distinct"
            );
        }
        for (zr, _) in pd_groups {
            assert!(
                xa[1..1 + n_log] != **zr,
                "an RS row point collides with a group's"
            );
        }
    }
    let mut ws = mp.statement_ws.iter();
    for (rc, &coeff) in rs.iter().zip(&mp.rs_coeffs) {
        if coeff.is_zero() {
            continue;
        }
        assert_eq!(
            *ws.next().expect("a statement per live RS claim"),
            coeff * rc.value,
            "RS statement recombination"
        );
    }
    for ((combo, dense), &coeff) in groups.iter().zip(&mp.group_coeffs) {
        let raw = combo.as_ref().map_or(F128::ZERO, |c| c.value)
            + dense
                .iter()
                .fold(F128::ZERO, |a, &(g, ref c)| a + g * c.value);
        assert_eq!(
            *ws.next().expect("a statement per group"),
            coeff * raw,
            "group statement recombination"
        );
    }
    assert!(ws.next().is_none(), "every statement accounted for");

    JaggedAssertion {
        k: params.k,
        m: params.m,
        rs,
        groups,
    }
}

pub fn verify_batch_merged<Ch: Challenger>(
    commitment: &Commitment,
    claims: &[F128],
    z_skips: &[SkipPoint],
    x_outers: &[&[F128]],
    packed_direct: &[PackedDirectClaimRef<'_>],
    heights: &[u64],
    n_log: usize,
    proof: &MergedOpenProof,
    lig_config: &VerifierConfig,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
) -> Result<(), PcsOpenError> {
    verify_batch_merged_core(
        commitment,
        claims,
        z_skips,
        x_outers,
        packed_direct,
        heights,
        n_log,
        proof,
        lig_config,
        grinding,
        challenger,
        None,
        None,
    )
}

/// Verify the production merged opening with complete 128 bit-column weights
/// for each ring-switch claim. All dense/jagged transport, Frobenius assist,
/// grinding and inner Ligerito checks use the same implementation as
/// [`verify_batch_merged`]. Only the scalar check against each `s_hat_v` takes
/// these weights instead of reconstructing skip64 weights.
///
/// Each `x_outers[i]` retains the legacy shape: an unused leading coordinate,
/// then `n_log` ordinary row coordinates and the column coordinates. Its
/// leading coordinate has no meaning on this weighted entry. The caller must
/// derive and bind every claim, weight and point from its public statement and
/// verified PIOP before this call; none are read from the opening proof.
#[allow(clippy::too_many_arguments)]
pub fn verify_batch_merged_weighted<Ch: Challenger>(
    commitment: &Commitment,
    claims: &[F128],
    columns: &[&[F128]],
    x_outers: &[&[F128]],
    packed_direct: &[PackedDirectClaimRef<'_>],
    heights: &[u64],
    n_log: usize,
    proof: &MergedOpenProof,
    lig_config: &VerifierConfig,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
) -> Result<(), PcsOpenError> {
    verify_batch_merged_core(
        commitment,
        claims,
        &[],
        x_outers,
        packed_direct,
        heights,
        n_log,
        proof,
        lig_config,
        grinding,
        challenger,
        None,
        Some(columns),
    )
}

/// [`verify_batch_merged`] plus the jagged-layout export — the count win's
/// deferral seam. The anchor's count-dependent `W`-values come back as RAW
/// claims on the layout table ([`crate::matrix_fold::JaggedAssertion`]),
/// each tied to the verifier's own expect by an EXACT recombination assert
/// (field sums and products distribute, so the tie is equality, not
/// tolerance). Transcript-identical to the plain entry.
#[allow(clippy::too_many_arguments)]
pub fn verify_batch_merged_deferred<Ch: Challenger>(
    commitment: &Commitment,
    claims: &[F128],
    z_skips: &[SkipPoint],
    x_outers: &[&[F128]],
    packed_direct: &[PackedDirectClaimRef<'_>],
    heights: &[u64],
    n_log: usize,
    proof: &MergedOpenProof,
    lig_config: &VerifierConfig,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
) -> Result<JaggedAssertion, PcsOpenError> {
    let mut out = None;
    verify_batch_merged_core(
        commitment,
        claims,
        z_skips,
        x_outers,
        packed_direct,
        heights,
        n_log,
        proof,
        lig_config,
        grinding,
        challenger,
        Some(&mut out),
        None,
    )?;
    Ok(out.expect("the deferred core fills the export"))
}

#[allow(clippy::too_many_arguments)]
fn verify_batch_merged_core<Ch: Challenger>(
    commitment: &Commitment,
    claims: &[F128],
    z_skips: &[SkipPoint],
    x_outers: &[&[F128]],
    packed_direct: &[PackedDirectClaimRef<'_>],
    heights: &[u64],
    n_log: usize,
    proof: &MergedOpenProof,
    lig_config: &VerifierConfig,
    grinding: OpeningGrinding,
    challenger: &mut Ch,
    defer: Option<&mut Option<JaggedAssertion>>,
    columns: Option<&[&[F128]]>,
) -> Result<(), PcsOpenError> {
    let n_rs = claims.len();
    let n_pd = packed_direct.len();
    if let Some(columns) = columns {
        if columns.len() != n_rs
            || columns
                .iter()
                .any(|weights| weights.len() != 1 << LOG_PACKING)
            || x_outers.len() != n_rs
            || !heights.len().is_power_of_two()
            || x_outers
                .iter()
                .any(|x| x.len() != 1 + n_log + heights.len().ilog2() as usize)
            || proof
                .ring_switches
                .iter()
                .any(|p| p.s_hat_v.len() != 1 << LOG_PACKING)
        {
            return Err(PcsOpenError::RingSwitch(RingSwitchError::MalformedProof));
        }
    } else {
        assert_eq!(z_skips.len(), n_rs);
    }
    assert_eq!(x_outers.len(), n_rs);
    if proof.ring_switches.len() != n_rs {
        return Err(PcsOpenError::Assist);
    }
    let batch_bits = grinding.claim_batch_bits_for(n_rs + n_pd);
    let expected_batch_nonces = usize::from(batch_bits != 0);
    if proof.batching_nonces.len() != expected_batch_nonces {
        return Err(PcsOpenError::Assist);
    }
    // `VERIFY_TRACE` phase split. The Ligerito inner verify has its own
    // `LIG_VERIFY_TRACE`, but it is a small tail here — the jagged Frobenius
    // assist is the term that scales with the row/column split.
    let trace = var("VERIFY_TRACE").is_ok();
    let tfmt = |s: f64| -> String {
        let ms = s * 1000.0;
        if ms < 1.0 {
            format!("{:>8.2} µs", s * 1e6)
        } else {
            format!("{:>8.2} ms", ms)
        }
    };
    challenger.observe_label(b"flock-merged-open-v1");
    let t = Instant::now();
    let mut rs_outputs = Vec::with_capacity(n_rs);
    for i in 0..n_rs {
        let out = if let Some(columns) = columns {
            verify_succinct_weighted_with_grinding(
                claims[i],
                columns[i],
                &proof.ring_switches[i],
                grinding.ring_switch_bits,
                challenger,
            )
        } else {
            let skip_w = z_skips[i].weights(LOG_PACKING - 1);
            verify_succinct_with_grinding(
                claims[i],
                &skip_w,
                x_outers[i],
                &proof.ring_switches[i],
                grinding.ring_switch_bits,
                challenger,
            )
        }
        .map_err(PcsOpenError::RingSwitch)?;
        rs_outputs.push(out);
    }
    if trace {
        eprintln!(
            "        [vbm] ring_switch::verify_succinct ×{n_rs}: {}",
            tfmt(t.elapsed().as_secs_f64())
        );
    }
    // Packed-direct values are bound before one PoW protects the entire mixed
    // coefficient vector (total discrepancy degree one).
    // VALUE-ONLY absorb (v1) — see the prover-side intake for the argument:
    // the points are transcript-derived and verifier-recomputed, never prover
    // messages.
    for c in packed_direct {
        challenger.observe_f128(c.value);
    }
    let gammas_all = if batch_bits != 0 {
        challenger
            .verify_pow_and_sample_f128_vec(proof.batching_nonces[0], batch_bits, n_rs + n_pd)
            .ok_or(PcsOpenError::Assist)?
    } else {
        challenger.sample_f128_vec(n_rs + n_pd)
    };
    let (gammas, gammas_pd) = gammas_all.split_at(n_rs);
    let mut target = F128::ZERO;
    for (out, g) in rs_outputs.iter().zip(gammas.iter()) {
        target += *g * out.sumcheck_claim;
    }
    for (c, g) in packed_direct.iter().zip(gammas_pd.iter()) {
        target += *g * c.value;
    }

    let dense_log = commitment.params.m - LOG_PACKING;
    if proof.merged_rounds.len() != dense_log {
        return Err(PcsOpenError::VirtualOpen);
    }
    let expected_merged_nonces = if grinding.merged_round_bits == 0 {
        0
    } else {
        dense_log
    };
    if proof.merged_round_nonces.len() != expected_merged_nonces {
        return Err(PcsOpenError::VirtualOpen);
    }
    let mut running = target;
    let mut rho = Vec::with_capacity(dense_log);
    for (round, &(g_one, g_inf)) in proof.merged_rounds.iter().enumerate() {
        challenger.observe_f128(g_one);
        challenger.observe_f128(g_inf);
        let r = if grinding.merged_round_bits != 0 {
            challenger
                .verify_pow_and_sample_f128(
                    proof.merged_round_nonces[round],
                    grinding.merged_round_bits,
                )
                .ok_or(PcsOpenError::VirtualOpen)?
        } else {
            challenger.sample_f128()
        };
        running = fold_round_claim(running, g_one, g_inf, r);
        rho.push(r);
    }

    let t = Instant::now();
    let params = JaggedParams::from_heights(heights, n_log, dense_log);
    let k_cols = params.k;
    if jagged::rectangular_prefix_columns(heights, n_log).is_some() {
        // Block-first transport: the prover bound the k−1 block coordinates
        // first; put ρ back into coordinate order.
        rho.rotate_left(k_cols - 1);
    }
    if trace {
        eprintln!(
            "        [vbm] JaggedParams::from_heights (n_log={n_log}, dense_log={dense_log}, k_cols={k_cols}): {}",
            tfmt(t.elapsed().as_secs_f64())
        );
    }
    let t = Instant::now();
    // The claims' c_{i,j}: derived from the transcript (γ-scaled r''-eq
    // tensors → fold byte tables → linearized coefficients).
    let coeffs: Vec<Vec<F128>> = rs_outputs
        .iter()
        .zip(gammas.iter())
        .map(|(o, g)| {
            let scaled: Vec<F128> = o.eq_r_dprime.iter().map(|x| *g * *x).collect();
            linearized_coefficients(&build_fold_byte_table(&scaled))
        })
        .collect();
    let fclaims: Vec<FrobeniusClaim<'_>> = x_outers
        .iter()
        .zip(&coeffs)
        .map(|(x, c)| {
            assert_eq!(x.len(), 1 + n_log + k_cols, "point/row/col split mismatch");
            FrobeniusClaim {
                z_row: &x[1..1 + n_log],
                z_col: &x[1 + n_log..],
                coeffs: c,
            }
        })
        .collect();
    // The packed-direct claims collapse into the SAME merged-column scalar
    // groups the prover built (identical claim order → identical groups):
    // a group's fold map is the identity with its γ's baked into the cols,
    // so it carries one dual value and no 128-coefficient vector at all.
    for c in packed_direct.iter() {
        if c.point.len() != n_log + k_cols {
            return Err(PcsOpenError::Assist);
        }
    }
    let pd_groups = scalar_claim_groups(
        packed_direct.iter().map(|c| c.point),
        gammas_pd,
        n_log,
        k_cols,
    );
    let gclaims: Vec<ScalarGroupClaim<'_>> = pd_groups
        .iter()
        .map(|(z_row, cols)| ScalarGroupClaim { z_row, cols })
        .collect();
    if trace {
        eprintln!(
            "        [vbm] coeffs (fold byte tables ×{n_rs}) + {} scalar groups: {}",
            gclaims.len(),
            tfmt(t.elapsed().as_secs_f64())
        );
    }
    let t = Instant::now();
    #[cfg(feature = "mul-count")]
    let assist_start = snapshot();
    // Mirror the prover's FORK/JOIN (see the prover-side note): the assist
    // replays on a domain-separated child seeded here, the inner opening on
    // the parent, and the child merges after. The verifier stays sequential
    // — only the transcript forks — so the assist runs first and its `v` is
    // held for the `running` check below, which moved past the merge.
    let mut ch_a = challenger.fork(b"flock-par-assist-v1");
    let mut mp_defer = None;
    let v = if defer.is_some() {
        let (v, mp) = verify_multipoint_twisted_deferred_with_grinding(
            &params,
            &fclaims,
            &gclaims,
            &rho,
            &proof.frobenius,
            grinding.multipoint,
            &mut ch_a,
        )
        .ok_or(PcsOpenError::Assist)?;
        mp_defer = Some(mp);
        v
    } else {
        verify_multipoint_twisted_with_grinding(
            &params,
            &fclaims,
            &gclaims,
            &rho,
            &proof.frobenius,
            grinding.multipoint,
            &mut ch_a,
        )
        .ok_or(PcsOpenError::Assist)?
    };
    if let (Some(out), Some(mp)) = (defer, mp_defer) {
        *out = Some(assemble_jagged_assertion(
            &params,
            x_outers,
            packed_direct,
            gammas_pd,
            &pd_groups,
            n_log,
            &mp,
        ));
    }
    #[cfg(feature = "mul-count")]
    if var("MUL_TRACE").is_ok() {
        let e = snapshot();
        let invs = e.invs - assist_start.invs;
        let muls = (e.native_muls - assist_start.native_muls).saturating_sub(invs * MULS_PER_INV);
        println!(
            "  [mul]   of which jagged::verify_frobenius_assist:    {muls:>8} muls {invs:>5} invs \
             = {:>8} constraints",
            muls + invs
        );
    }
    if trace {
        eprintln!(
            "        [vbm] jagged::verify_frobenius_assist: {}",
            tfmt(t.elapsed().as_secs_f64())
        );
    }
    let t = Instant::now();
    let pd = PackedDirectClaimRef {
        point: &rho,
        value: proof.q_eval,
    };
    verify_opening_batch_ligerito_mixed_with_grinding(
        commitment,
        &[],
        &[],
        &[],
        &[pd],
        &proof.inner,
        lig_config,
        grinding,
        challenger,
    )
    .map_err(|_| PcsOpenError::Ligerito)?;
    challenger.merge_child(ch_a);
    // The assist's claim against the merged sumcheck's folded target. A pure
    // arithmetic check — it consumes no challenges, so the fork moved it here
    // without touching the transcript.
    if running != proof.q_eval * v {
        return Err(PcsOpenError::VirtualOpen);
    }
    if trace {
        eprintln!(
            "        [vbm] verify_opening_batch_ligerito_mixed: {}",
            tfmt(t.elapsed().as_secs_f64())
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        challenger::FsChallenger,
        pcs::{
            F128, LOG_PACKING, OpeningGrinding, PaddingSpec, PcsError, PcsParams, RingSwitchError,
            commit,
            ligerito::{LigeritoProfile, prover_config_for, verifier_config_for},
            open_batch_mixed_ligerito_with_precomputed_s_hat_v_and_grinding, pack_witness,
            verify_opening_batch_ligerito_mixed_with_grinding,
        },
        test_rng::Rng,
        zerocheck::{multilinear::lagrange_weights_naive, univariate_skip::build_eq},
    };

    #[test]
    fn direct_block_parallel_statistics_match_reference() {
        use super::{
            MergedRowEq, direct_block_dot, direct_block_statistics,
            with_direct_block_parallel_override,
        };
        let mut rng = Rng::new(0xD1_B10C_01);
        // Unaligned SIMD loads, empty inputs and scalar tails agree with
        // individual reduced products, including the parallel chunk boundary.
        for len in [0, 1, 2, 3, 4, 5, 7, 8, 9, 8191, 8192, 8193] {
            let words = rng.f128_vec(len + 1);
            let weights = rng.f128_vec(len + 1);
            let expected = words[1..]
                .iter()
                .zip(&weights[1..])
                .fold(F128::ZERO, |a, (&x, &y)| a + x * y);
            assert_eq!(direct_block_dot(&words[1..], &weights[1..]), expected);
        }
        for (n, k) in [(0, 3), (1, 4), (3, 1), (9, 2), (15, 1)] {
            let q = rng.f128_vec(1 << (n + k));
            let mut point = rng.f128_vec(n);
            if n > 1 {
                point[0] = F128::ZERO;
                point[n - 1] = F128::ONE;
            }
            let dense = MergedRowEq::Dense(super::build_eq_table(&point));
            // Explicit asymmetric splits cover n_lo=0, n_lo=n and lo tables
            // larger than a chunk, without allocating a large test domain.
            for n_lo in [0, n / 2, n] {
                let (lo, hi) = super::ring_switch::build_eq_split(&point, n_lo);
                let split = MergedRowEq::Split { lo, hi, n_lo };
                for live in [1, (1usize << k).div_ceil(2), 1 << k] {
                    // Dirty/nonexistent inactive columns must never be read.
                    let live_q = &q[..live << n];
                    let expected = with_direct_block_parallel_override(false, || {
                        direct_block_statistics(live_q, n, k, live, &dense)
                    });
                    for eq in [&dense, &split] {
                        let actual = with_direct_block_parallel_override(true, || {
                            direct_block_statistics(live_q, n, k, live, eq)
                        });
                        assert_eq!(actual, expected, "n={n} k={k} live={live} n_lo={n_lo}");
                    }
                }
            }
        }
    }

    #[derive(Clone, Copy)]
    enum DirectEngineering {
        BlockParallel,
        DirectWp,
        UninitWp(bool),
    }

    fn engineering_transports(arm: DirectEngineering) -> &'static [(bool, bool)] {
        match arm {
            DirectEngineering::UninitWp(_) => &[(true, false), (false, true), (true, true)],
            _ => &[(true, true)],
        }
    }

    fn with_direct_engineering_override<T>(
        arm: DirectEngineering,
        enabled: bool,
        f: impl FnOnce() -> T,
    ) -> T {
        let (parallel, simd, uninit) = match arm {
            DirectEngineering::BlockParallel => (enabled, false, false),
            DirectEngineering::DirectWp => (true, enabled, false),
            DirectEngineering::UninitWp(simd) => (true, simd, enabled),
        };
        super::with_direct_block_parallel_override(parallel, || {
            super::with_direct_wp_simd_override(simd, || super::with_wp_uninit_override(uninit, f))
        })
    }

    #[test]
    fn wp_uninit_chunks_initialize_exact_values_and_unwind_safely() {
        use super::{MergedRowEq, initialize_wp_chunk};
        use rayon::prelude::*;
        let mut rng = Rng::new(0x71_1A17);
        for n in [0, 1, 5, 12] {
            let points = [rng.f128_vec(n), vec![F128::ZERO; n]];
            for n_lo in [0, n / 2, n] {
                let split_eq = |point: &[F128]| {
                    let (lo, hi) = super::ring_switch::build_eq_split(point, n_lo);
                    MergedRowEq::Split { lo, hi, n_lo }
                };
                let eq_rows = [
                    MergedRowEq::Dense(super::build_eq_table(&points[0])),
                    split_eq(&points[1]),
                ];
                let tabs: Vec<[Vec<F128>; 2]> = (0..2)
                    .map(|j| {
                        std::array::from_fn(|c0| {
                            let weights = if j == c0 {
                                vec![F128::ZERO; 128]
                            } else {
                                rng.f128_vec(128)
                            };
                            super::ring_switch::build_fold_byte_table(&weights)
                        })
                    })
                    .collect();
                let pd = [
                    (split_eq(&points[0]), [F128::ZERO, rng.f128()]),
                    (split_eq(&points[1]), [rng.f128(), F128::ONE]),
                ];
                for (use_rs, use_pd) in [(false, false), (true, false), (false, true), (true, true)]
                {
                    let rows = if use_rs { &eq_rows[..] } else { &[] };
                    let groups = if use_pd { &pd[..] } else { &[] };
                    let expected: Vec<_> = (0..2 << n)
                        .map(|h| {
                            let row = h & ((1 << n) - 1);
                            let c0 = h >> n;
                            let mut value = F128::ZERO;
                            for (eq, tab) in rows.iter().zip(&tabs) {
                                value += super::ring_switch::fold_one_slot(eq.at(row), &tab[c0]);
                            }
                            for (eq, scale) in groups {
                                value += scale[c0] * eq.at(row);
                            }
                            value
                        })
                        .collect();
                    for simd in [false, true] {
                        for chunk_size in [1, 3, 7, 2048, 2051] {
                            let mut output = Vec::<F128>::with_capacity(expected.len() + 7);
                            output.spare_capacity_mut()[..expected.len()]
                                .par_chunks_mut(chunk_size)
                                .enumerate()
                                .for_each(|(i, chunk)| {
                                    initialize_wp_chunk(
                                        chunk,
                                        i * chunk_size,
                                        n,
                                        rows,
                                        &tabs,
                                        groups,
                                        simd,
                                    );
                                });
                            assert_eq!(output.len(), 0);
                            // SAFETY: every requested slot was written by joined workers.
                            unsafe { output.set_len(expected.len()) };
                            assert_eq!(
                                output, expected,
                                "n={n} n_lo={n_lo} chunk={chunk_size} simd={simd}"
                            );
                        }
                    }
                }
            }
        }
        // Deliberately panic after writing the first slot. The remaining
        // storage is never exposed as F128, and its owner remains length zero.
        let mut output = Vec::<F128>::with_capacity(8);
        let short_eq = [MergedRowEq::Dense(vec![F128::ONE])];
        let tabs = [[vec![F128::ZERO; 4096], vec![F128::ZERO; 4096]]];
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            initialize_wp_chunk(
                &mut output.spare_capacity_mut()[..8],
                0,
                2,
                &short_eq,
                &tabs,
                &[],
                true,
            );
        }));
        assert!(failed.is_err());
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn direct_wp_simd_segments_match_scalar() {
        use super::{MergedRowEq, direct_wp_add_groups, direct_wp_scaled_add};
        let mut rng = Rng::new(0xD1_7001);
        for len in [0, 1, 2, 3, 4, 5, 7, 8, 9, 2047, 2048, 2049] {
            let initial = rng.f128_vec(len + 1);
            let weights = rng.f128_vec(len + 1);
            for scale in [F128::ZERO, F128::ONE, rng.f128()] {
                let mut expected = initial.clone();
                for (slot, &weight) in expected[1..].iter_mut().zip(&weights[1..]) {
                    *slot += scale * weight;
                }
                let mut actual = initial.clone();
                direct_wp_scaled_add(&mut actual[1..], &weights[1..], scale);
                assert_eq!(actual, expected);
            }
        }
        for n in [0, 1, 2, 5, 12, 15] {
            let mut points = vec![rng.f128_vec(n), rng.f128_vec(n), vec![F128::ZERO; n]];
            if n > 0 {
                points[1][n - 1] = F128::ONE;
            }
            let scales = [
                [F128::ZERO, rng.f128()],
                [rng.f128(), F128::ONE],
                [F128::ONE, rng.f128()],
            ];
            let initial = rng.f128_vec(2 << n);
            for n_lo in [0, n / 2, n] {
                let groups: Vec<_> = points
                    .iter()
                    .enumerate()
                    .map(|(i, point)| {
                        let row_eq = if i == 0 {
                            MergedRowEq::Dense(super::build_eq_table(point))
                        } else {
                            let (lo, hi) = super::ring_switch::build_eq_split(point, n_lo);
                            MergedRowEq::Split { lo, hi, n_lo }
                        };
                        (row_eq, scales[i])
                    })
                    .collect();
                let mut expected = initial.clone();
                for (h, slot) in expected.iter_mut().enumerate() {
                    for (eq, scale) in &groups {
                        *slot += scale[h >> n] * eq.at(h & ((1 << n) - 1));
                    }
                }
                // Odd offsets and chunk sizes cross low/high split boundaries
                // and the c0 boundary; production uses aligned 2048 chunks.
                for chunk_size in [1, 3, 5, 2048, 2051] {
                    let mut actual = initial.clone();
                    for (i, chunk) in actual.chunks_mut(chunk_size).enumerate() {
                        direct_wp_add_groups(chunk, i * chunk_size, n, &groups);
                    }
                    assert_eq!(actual, expected, "n={n} n_lo={n_lo} chunk={chunk_size}");
                }
            }
        }
    }

    #[test]
    fn direct_block_parallel_phase_matches_reference() {
        check_direct_engineering_phase(DirectEngineering::BlockParallel);
    }

    #[test]
    fn direct_wp_simd_phase_matches_reference() {
        check_direct_engineering_phase(DirectEngineering::DirectWp);
    }

    #[test]
    fn wp_uninit_phase_matches_initialized_with_both_direct_paths() {
        for simd in [false, true] {
            check_direct_engineering_phase(DirectEngineering::UninitWp(simd));
        }
    }

    fn check_direct_engineering_phase(arm: DirectEngineering) {
        use super::{BlockFirstClaim, block_first_phase, column_bitbank_reference};
        use crate::challenger::Challenger;
        let mut rng = Rng::new(0xD1_B10C_02);
        for (n, k) in [(0, 2), (3, 5), (9, 1), (10, 3)] {
            let q = rng.f128_vec(1 << (n + k));
            let rows = [rng.f128_vec(n), rng.f128_vec(n), vec![F128::ZERO; n]];
            let cols = rng.f128_vec(k);
            let coeffs = rng.f128_vec(128);
            let bank = column_bitbank_reference(&q, &rows[0], n, k);
            let rs = [BlockFirstClaim {
                z_row: &rows[0],
                z_col: &cols,
                coeffs: &coeffs,
                bank: &bank,
            }];
            let mut groups: Vec<_> = rows
                .iter()
                .map(|row| (row.as_slice(), rng.f128_vec(1 << k)))
                .collect();
            // Group coefficients can vanish or be one-hot, but statistics
            // cannot prune those columns before the block cross terms.
            groups[1].1.fill(F128::ZERO);
            groups[2].1.fill(F128::ZERO);
            groups[2].1[(1 << k) - 1] = F128::ONE;
            for &(use_rs, use_pd) in engineering_transports(arm) {
                for live in [1, (1 << k) - 1, 1 << k] {
                    for split in [false, true] {
                        let run = |parallel| {
                            super::with_merged_split_rows_override(split, || {
                                with_direct_engineering_override(arm, parallel, || {
                                    let mut ch = FsChallenger::new(b"direct-block-parallel-phase");
                                    let (mut rounds, mut nonces, mut point) =
                                        (Vec::new(), Vec::new(), Vec::new());
                                    let (qp, wp) = block_first_phase(
                                        &q,
                                        n,
                                        k,
                                        live,
                                        &rs[..usize::from(use_rs)],
                                        if use_pd { &groups } else { &[] },
                                        OpeningGrinding::disabled(),
                                        &mut ch,
                                        &mut rounds,
                                        &mut nonces,
                                        &mut point,
                                        false,
                                    );
                                    (qp, wp, rounds, nonces, point, ch.sample_f128())
                                })
                            })
                        };
                        assert_eq!(
                            run(false),
                            run(true),
                            "n={n} k={k} live={live} split={split}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn direct_block_parallel_full_native_mixed_proof() {
        check_direct_engineering_full_native_proof(DirectEngineering::BlockParallel);
    }

    #[test]
    fn direct_wp_simd_full_native_mixed_proof() {
        check_direct_engineering_full_native_proof(DirectEngineering::DirectWp);
    }

    #[test]
    fn wp_uninit_full_native_proof_and_challenge_all_routes() {
        for simd in [false, true] {
            check_direct_engineering_full_native_proof(DirectEngineering::UninitWp(simd));
        }
    }

    fn check_direct_engineering_full_native_proof(arm: DirectEngineering) {
        use super::{
            DirectEqInd, PackedDirectClaim, PackedDirectClaimRef, open_batch_merged,
            verify_batch_merged_weighted,
        };
        use crate::challenger::Challenger;
        let mut rng = Rng::new(0xD1_B10C_03);
        // Four and 128 word columns exercise different block-round counts;
        // direct row groups include one shared with the ring-switch claim.
        for k in [2, 7] {
            let n = 15 - k;
            let q = rng.f128_vec(1 << 15);
            let params = PcsParams {
                m: 22,
                log_inv_rate: 1,
                log_batch_size: 6,
                profile: LigeritoProfile::Fast,
                num_lanes: None,
                merkle_hash: Default::default(),
            };
            let (cm, data) = commit(&q, &params);
            let x = rng.f128_vec(16);
            let columns = rng.f128_vec(128);
            let s_hat = super::ring_switch::fold_1b_rows_naive(&q, &build_eq(&x[1..]));
            let value = super::ring_switch::claim_check(&columns, &s_hat);
            let mut points = vec![
                x[1..].to_vec(),
                rng.f128_vec(15),
                rng.f128_vec(15),
                x[1..].to_vec(),
            ];
            points[3][14] = F128::ZERO;
            let direct: Vec<_> = points
                .into_iter()
                .map(|point| {
                    let mut words = q.clone();
                    for &r in &point {
                        words = words
                            .chunks_exact(2)
                            .map(|p| p[0] + r * (p[0] + p[1]))
                            .collect();
                    }
                    PackedDirectClaim {
                        value: words[0],
                        eq_ind: DirectEqInd::EqPoint(point.clone()),
                        point,
                    }
                })
                .collect();
            let refs: Vec<_> = direct
                .iter()
                .map(|claim| PackedDirectClaimRef {
                    point: &claim.point,
                    value: claim.value,
                })
                .collect();
            let heights = vec![1u64 << n; 1 << k];
            for &(use_rs, use_pd) in engineering_transports(arm) {
                let rs_count = usize::from(use_rs);
                let points = [x.as_slice()];
                let precomputed = [Some(s_hat.as_slice())];
                let rs_columns = [columns.as_slice()];
                for split in [false, true] {
                    let run = |parallel| {
                        super::with_merged_split_rows_override(split, || {
                            with_direct_engineering_override(arm, parallel, || {
                                let mut ch = FsChallenger::new(b"direct-block-parallel-native");
                                let proof = open_batch_merged(
                                    q.clone(),
                                    None,
                                    &data,
                                    &cm,
                                    &points[..rs_count],
                                    &precomputed[..rs_count],
                                    &[],
                                    if use_pd { &direct } else { &[] },
                                    &PaddingSpec::dense(22),
                                    &heights,
                                    n,
                                    &params.ligerito_prover_config().unwrap(),
                                    OpeningGrinding::disabled(),
                                    &mut ch,
                                );
                                (proof, ch.sample_f128())
                            })
                        })
                    };
                    let (old, old_after) = run(false);
                    let (new, after) = run(true);
                    assert_eq!(
                        bincode::serialize(&old).unwrap(),
                        bincode::serialize(&new).unwrap()
                    );
                    assert_eq!(old_after, after);
                    let mut vch = FsChallenger::new(b"direct-block-parallel-native");
                    verify_batch_merged_weighted(
                        &cm,
                        &[value][..rs_count],
                        &rs_columns[..rs_count],
                        &points[..rs_count],
                        if use_pd { &refs } else { &[] },
                        &heights,
                        n,
                        &new,
                        &params.ligerito_verifier_config().unwrap(),
                        OpeningGrinding::disabled(),
                        &mut vch,
                    )
                    .unwrap();
                    assert_eq!(after, vch.sample_f128());
                }
            }
        }
    }

    #[test]
    fn merged_row_grouping_block_first_matches_uncombined_maps() {
        use super::{BlockFirstClaim, block_first_phase, column_bitbank_split};
        use crate::challenger::Challenger;
        let mut rng = Rng::new(0xB10C_60A0);
        let (n, k) = (6, 3);
        let q = rng.f128_vec(1 << (n + k));
        let first = rng.f128_vec(n);
        let rows = [first.clone(), first, rng.f128_vec(n)];
        let cols = [
            rng.f128_vec(k),
            vec![F128::ZERO, F128::ONE, F128::ZERO],
            rng.f128_vec(k),
        ];
        let coeffs = [rng.f128_vec(128), rng.f128_vec(128), rng.f128_vec(128)];
        let banks: Vec<_> = rows
            .iter()
            .map(|row| column_bitbank_split(&q, row, n, k))
            .collect();
        let rs: Vec<_> = (0..3)
            .map(|i| BlockFirstClaim {
                z_row: &rows[i],
                z_col: &cols[i],
                coeffs: &coeffs[i],
                bank: &banks[i],
            })
            .collect();
        let groups = vec![(rows[0].as_slice(), rng.f128_vec(1 << k))];
        for live_cols in [1, 3, 8] {
            let prove = |grouping| {
                super::with_merged_row_grouping_override(grouping, || {
                    let mut ch = FsChallenger::new(b"block-first-row-map-grouping");
                    let (mut rounds, mut nonces, mut point) = (Vec::new(), Vec::new(), Vec::new());
                    let (qp, wp) = block_first_phase(
                        &q,
                        n,
                        k,
                        live_cols,
                        &rs,
                        &groups,
                        OpeningGrinding::disabled(),
                        &mut ch,
                        &mut rounds,
                        &mut nonces,
                        &mut point,
                        false,
                    );
                    (qp, wp, rounds, nonces, point, ch.sample_f128())
                })
            };
            assert_eq!(prove(false), prove(true));
        }
    }

    #[test]
    fn merged_row_grouping_full_native_proof_and_transcript() {
        use crate::{
            challenger::Challenger,
            pcs::{
                functional::{TensorClaim, row_weights},
                merged_tensor, ring_switch,
            },
            r1cs::SparseBinaryMatrix,
            schedule::{Registry, TableClass, TableType},
            union::UnionInstance,
        };
        for (col_log, partial) in [(0, false), (2, false), (7, false), (2, true)] {
            let nu = 15 - col_log;
            let registry = Registry::new(
                vec![TableType {
                    k_log: 7 + col_log,
                    useful_bits: 128 << col_log,
                    a_0: SparseBinaryMatrix::new(0, 0, vec![]),
                    b_0: SparseBinaryMatrix::new(0, 0, vec![]),
                    c_0: SparseBinaryMatrix::new(0, 0, vec![]),
                    const_pin: None,
                    class: TableClass::Boolean,
                    io_schema: vec![],
                }],
                nu,
            );
            let count = (1 << nu) - usize::from(partial);
            let union = UnionInstance::new(&registry, vec![count]);
            let params = PcsParams {
                m: 22,
                log_inv_rate: 1,
                log_batch_size: 6,
                profile: LigeritoProfile::Fast,
                num_lanes: union.commit_lanes(6),
                merkle_hash: Default::default(),
            };
            let mut rng = Rng::new(0x60A0_51A7);
            let mut padded = rng.f128_vec(union.packed_len());
            // Actual constant columns exercise the online detection path;
            // the partial case still appends its canonical zero slot below.
            padded[..1 << nu].fill(F128::new(0x5A5A_A5A5, 0xDEAD_BEEF));
            if col_log >= 2 {
                padded[1 << nu..2 << nu].fill(F128::ZERO);
                padded[2 << nu..3 << nu].fill(F128::new(u64::MAX, u64::MAX));
            }
            if partial {
                for col in 0..1 << col_log {
                    padded[(col << nu) + count] = F128::ZERO;
                }
            }
            let q = union.compact_witness(&padded);
            let (cm, data) = commit(&q, &params);
            let mut claims: Vec<_> = (0..3)
                .map(|_| TensorClaim {
                    columns: rng.f128_vec(128),
                    rows: (0..15)
                        .map(|_| {
                            let r = rng.f128();
                            [F128::ONE + r, r]
                        })
                        .collect(),
                    value: F128::ZERO,
                })
                .collect();
            let repeated = claims[0].rows[..nu].to_vec();
            claims[1].rows[..nu].copy_from_slice(&repeated);
            claims[2].rows[2] = [F128::ONE; 2];
            // Opposite column parities share the first row point; the last
            // point requests only even columns. High-bit zeros must not
            // prune their cross terms in the block sumcheck.
            for (i, claim) in claims.iter_mut().enumerate() {
                for bit in 0..col_log {
                    let v = if i & (1 << bit) == 0 {
                        F128::ZERO
                    } else {
                        F128::ONE
                    };
                    claim.rows[nu + bit] = [F128::ONE + v, v];
                }
            }
            for claim in &mut claims {
                claim.value = ring_switch::claim_check(
                    &claim.columns,
                    &ring_switch::fold_1b_rows_naive(&padded, &row_weights(&claim.rows)),
                );
            }
            let prove = |grouping| {
                super::with_merged_row_grouping_override(grouping, || {
                    let mut ch = FsChallenger::new(b"merged-row-map-grouping-full");
                    let proof = if partial {
                        merged_tensor::open(
                            q.clone(),
                            &padded,
                            &data,
                            &cm,
                            &claims,
                            &union,
                            &mut ch,
                        )
                    } else {
                        merged_tensor::open_identity(
                            q.clone(),
                            &data,
                            &cm,
                            &claims,
                            &union,
                            &mut ch,
                        )
                    }
                    .unwrap();
                    (proof, ch.sample_f128())
                })
            };
            let (old, old_next) = super::with_constant_bitbanks_override(false, || {
                super::with_sparse_bitbanks_override(false, || {
                    super::with_paired_bitbanks_override(false, || prove(false))
                })
            });
            let (grouped, grouped_next) = super::with_constant_bitbanks_override(false, || {
                super::with_sparse_bitbanks_override(false, || {
                    super::with_paired_bitbanks_override(false, || prove(true))
                })
            });
            let (new, new_next) = super::with_constant_bitbanks_override(true, || {
                super::with_sparse_bitbanks_override(true, || {
                    super::with_paired_bitbanks_override(true, || prove(true))
                })
            });
            assert_eq!(
                bincode::serialize(&old).unwrap(),
                bincode::serialize(&grouped).unwrap()
            );
            assert_eq!(old_next, grouped_next);
            assert_eq!(
                bincode::serialize(&old).unwrap(),
                bincode::serialize(&new).unwrap()
            );
            assert_eq!(old_next, new_next);
            let mut ch = FsChallenger::new(b"merged-row-map-grouping-full");
            merged_tensor::verify(&cm, &claims, &new, &union, &mut ch).unwrap();
            assert_eq!(new_next, ch.sample_f128());
        }
    }

    #[test]
    fn paired_bitbanks_match_independent_folds_and_cache_order() {
        let mut rng = Rng::new(0xBA01_2A77);
        for (n, k) in [(3, 0), (4, 2), (7, 7), (11, 2)] {
            let mut q = rng.f128_vec(1 << (n + k));
            for (i, word) in q.iter_mut().enumerate() {
                if i % 17 == 0 {
                    *word = F128::ZERO;
                } else if i % 23 == 0 {
                    *word = F128::new(u64::MAX, u64::MAX);
                }
            }
            let mut rows = [rng.f128_vec(n), rng.f128_vec(n)];
            rows[0][0] = F128::ZERO;
            rows[1][0] = F128::ONE;
            let pair = super::column_bitbank_split_pair(&q, [&rows[0], &rows[1]], n, k);
            for i in 0..2 {
                assert_eq!(pair[i], super::column_bitbank_split(&q, &rows[i], n, k));
            }
        }
        let (n, k) = (7, 2);
        let q = rng.f128_vec(1 << (n + k));
        let mut points: Vec<_> = (0..5).map(|_| rng.f128_vec(1 + n + k)).collect();
        let first = points[0][1..1 + n].to_vec();
        points[1][1..1 + n].copy_from_slice(&first);
        let third = points[2][1..1 + n].to_vec();
        points[4][1..1 + n].copy_from_slice(&third);
        let xs: Vec<_> = points.iter().map(Vec::as_slice).collect();
        let supplied = super::column_bitbank_split(&q, &first, n, k);
        let run = |paired| {
            super::with_paired_bitbanks_override(paired, || {
                super::column_bitbanks_for_claims(
                    &q,
                    &xs,
                    &[Some(supplied.as_slice())],
                    n,
                    k,
                    false,
                )
            })
        };
        let old = run(false);
        let new = run(true);
        assert_eq!(old, new);
        assert_eq!(new.1, vec![0, 0, 1, 2, 1]);
        assert!(matches!(&new.0[0], std::borrow::Cow::Borrowed(_)));
    }
    #[test]
    fn sparse_bitbanks_masks_pairs_and_unions_preserve_required_columns() {
        let mut rng = Rng::new(0x5A12_5EBA);
        for (n, k) in [(3, 0), (4, 2), (7, 7), (11, 2)] {
            let q = rng.f128_vec(1 << (n + k));
            let rows = [rng.f128_vec(n), rng.f128_vec(n)];
            let full = rows
                .each_ref()
                .map(|r| super::column_bitbank_split(&q, r, n, k));
            for masks in [
                [[true, false], [false, true]],
                [[true; 2], [true, false]],
                [[false, true]; 2],
            ] {
                let pair = super::column_bitbank_split_pair_masked(
                    &q,
                    [&rows[0], &rows[1]],
                    n,
                    k,
                    masks,
                    &[],
                );
                for i in 0..2 {
                    assert_eq!(
                        pair[i],
                        super::column_bitbank_split_masked(&q, &rows[i], n, k, masks[i], &[])
                    );
                    for c in 0..1 << k {
                        let range = c * 128..(c + 1) * 128;
                        if masks[i][c & 1] {
                            assert_eq!(pair[i][range.clone()], full[i][range]);
                        } else {
                            assert!(pair[i][range].iter().all(|x| *x == F128::ZERO));
                        }
                    }
                }
            }
        }
        let (n, k) = (7, 3);
        let q = rng.f128_vec(1 << (n + k));
        let mut points: Vec<_> = (0..6).map(|_| rng.f128_vec(1 + n + k)).collect();
        let first = points[0][1..1 + n].to_vec();
        points[1][1..1 + n].copy_from_slice(&first);
        points[5][1..1 + n].copy_from_slice(&first);
        let third = points[2][1..1 + n].to_vec();
        points[4][1..1 + n].copy_from_slice(&third);
        for (i, point) in points.iter_mut().take(5).enumerate() {
            point[1 + n] = if i == 1 || i == 3 {
                F128::ONE
            } else {
                F128::ZERO
            };
        }
        points[5][1 + n] = F128::new(2, 0);
        let xs: Vec<_> = points.iter().map(Vec::as_slice).collect();
        let supplied = super::column_bitbank_split(&q, &first, n, k);
        for (paired, borrow) in [(false, false), (true, false), (false, true), (true, true)] {
            let run = |sparse| {
                super::with_sparse_bitbanks_override(sparse, || {
                    super::with_paired_bitbanks_override(paired, || {
                        let pre = [Some(supplied.as_slice())];
                        super::column_bitbanks_for_claims(
                            &q,
                            &xs,
                            if borrow { &pre } else { &[] },
                            n,
                            k,
                            false,
                        )
                    })
                })
            };
            let (old, order) = run(false);
            let (new, new_order) = run(true);
            assert_eq!(order, new_order);
            assert_eq!(matches!(&new[0], std::borrow::Cow::Borrowed(_)), borrow);
            for (claim, point) in points.iter().enumerate() {
                let mask = super::bitbank_parities(&point[1 + n..]);
                for c in 0..1 << k {
                    if mask[c & 1] {
                        let range = c * 128..(c + 1) * 128;
                        assert_eq!(old[order[claim]][range.clone()], new[order[claim]][range]);
                    }
                }
                assert_eq!(
                    super::column_bank_s_hat(&old[order[claim]], &point[1 + n..]),
                    super::column_bank_s_hat(&new[order[claim]], &point[1 + n..])
                );
            }
        }
    }

    #[test]
    fn sparse_bitbanks_block_messages_need_high_column_cross_terms() {
        use super::{BlockFirstClaim, block_first_phase};
        use crate::challenger::Challenger;
        let mut rng = Rng::new(0xC205_57E2);
        let (n, k) = (6, 3);
        let q = rng.f128_vec(1 << (n + k));
        let row = rng.f128_vec(n);
        let col = [F128::ZERO, F128::ONE, F128::ZERO]; // column 2
        let coeffs = rng.f128_vec(128);
        let full = super::column_bitbank_split(&q, &row, n, k);
        let parity =
            super::column_bitbank_split_masked(&q, &row, n, k, super::bitbank_parities(&col), &[]);
        let mut only_hot_column = vec![F128::ZERO; full.len()];
        only_hot_column[256..384].copy_from_slice(&full[256..384]);
        let run = |bank: &[F128], sparse| {
            super::with_sparse_bitbanks_override(sparse, || {
                let rs = [BlockFirstClaim {
                    z_row: &row,
                    z_col: &col,
                    coeffs: &coeffs,
                    bank,
                }];
                let mut ch = FsChallenger::new(b"sparse-bank-cross-terms");
                let (mut rounds, mut nonces, mut point) = (Vec::new(), Vec::new(), Vec::new());
                let (qp, wp) = block_first_phase(
                    &q,
                    n,
                    k,
                    1 << k,
                    &rs,
                    &[],
                    OpeningGrinding::disabled(),
                    &mut ch,
                    &mut rounds,
                    &mut nonces,
                    &mut point,
                    false,
                );
                (qp, wp, rounds, nonces, point, ch.sample_f128())
            })
        };
        let expected = run(&full, false);
        assert_eq!(expected, run(&parity, true));
        assert_ne!(
            expected.2,
            run(&only_hot_column, true).2,
            "one-hot input support does not eliminate high-column cross terms"
        );
    }

    #[test]
    fn constant_bitbanks_check_actual_columns_and_preserve_every_row_point() {
        let mut rng = Rng::new(0xC057_A47C);
        let (n, k) = (8, 3);
        let n_rows = 1usize << n;
        let mut q = rng.f128_vec(1 << (n + k));
        let values = [
            F128::ZERO,
            F128::ONE,
            rng.f128(),
            F128::new(u64::MAX, u64::MAX),
            rng.f128(),
        ];
        for (col, value) in values.into_iter().enumerate() {
            q[col * n_rows..(col + 1) * n_rows].fill(value);
        }
        // A late mismatch must never be mistaken for a constant.
        q[5 * n_rows - 1] += F128::ONE;
        let constants = super::constant_bitbank_columns(&q, n, k);
        assert_eq!(
            &constants[..4],
            &values[..4].iter().copied().map(Some).collect::<Vec<_>>()
        );
        assert!(constants[4..].iter().all(Option::is_none));
        let mut points: Vec<_> = (0..3).map(|_| rng.f128_vec(1 + n + k)).collect();
        points[0][1..1 + n].fill(F128::ZERO);
        points[1][1..1 + n].fill(F128::ONE);
        points[0][1 + n] = F128::ZERO;
        points[1][1 + n] = F128::ONE;
        let xs: Vec<_> = points.iter().map(Vec::as_slice).collect();
        for paired in [false, true] {
            for sparse in [false, true] {
                let run = |constant| {
                    super::with_constant_bitbanks_override(constant, || {
                        super::with_sparse_bitbanks_override(sparse, || {
                            super::with_paired_bitbanks_override(paired, || {
                                super::column_bitbanks_for_claims(&q, &xs, &[], n, k, false)
                            })
                        })
                    })
                };
                assert_eq!(run(false), run(true));
            }
        }
        // The same rule applies when every column is constant, with no
        // random fallback column supplying accidentally missing values.
        q.fill(values[2]);
        let constants = super::constant_bitbank_columns(&q, n, k);
        assert!(constants.iter().all(|x| *x == Some(values[2])));
        for point in points {
            let expected = super::column_bitbank_split(&q, &point[1..1 + n], n, k);
            let actual = super::column_bitbank_split_masked(
                &q,
                &point[1..1 + n],
                n,
                k,
                [true; 2],
                &constants,
            );
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn column_bank_s_hat_matches_full_ring_switch_fold() {
        let mut rng = Rng::new(0x5A7_BA0C);
        for (n, k) in [(3, 0), (4, 2), (4, 7), (8, 2)] {
            let q = rng.f128_vec(1 << (n + k));
            let mut row = rng.f128_vec(n);
            row[0] = F128::ONE;
            let mut column = rng.f128_vec(k);
            if k > 0 {
                column[0] = F128::ZERO;
            }
            let bank = super::column_bitbank_split(&q, &row, n, k);
            let mut whole_point = row;
            whole_point.extend(&column);
            let expected = super::ring_switch::fold_1b_rows_naive(&q, &build_eq(&whole_point));
            assert_eq!(super::column_bank_s_hat(&bank, &column), expected);
        }
    }

    #[test]
    fn full_rectangle_bank_reuse_requires_complete_support() {
        use super::full_rectangle_without_padding;
        let dense = PaddingSpec::dense(17);
        assert!(full_rectangle_without_padding(1024, &[256; 4], 8, &dense));
        assert!(full_rectangle_without_padding(
            1024,
            &[1024],
            10,
            &PaddingSpec::uniform(7, 128, 1024)
        ));
        assert!(!full_rectangle_without_padding(
            1024,
            &[255, 256, 256, 256],
            8,
            &dense
        ));
        assert!(!full_rectangle_without_padding(1024, &[256; 3], 8, &dense));
        assert!(!full_rectangle_without_padding(1023, &[256; 4], 8, &dense));
        assert!(!full_rectangle_without_padding(
            1024,
            &[256; 4],
            8,
            &PaddingSpec::uniform(7, 127, 1024)
        ));
        assert!(!full_rectangle_without_padding(
            1024,
            &[256; 4],
            8,
            &PaddingSpec::dense(16)
        ));
    }

    #[test]
    fn column_bitbank_split_matches_scalar_across_shapes() {
        let mut rng = Rng::new(0xB17_BA0C);
        for (n, k) in [(0, 0), (3, 2), (4, 7), (7, 2), (11, 0), (10, 7)] {
            let mut q = rng.f128_vec(1 << (n + k));
            for (i, word) in q.iter_mut().enumerate() {
                if i % 17 == 0 {
                    *word = F128::ZERO;
                } else if i % 23 == 0 {
                    *word = F128::new(u64::MAX, u64::MAX);
                }
            }
            let mut point = rng.f128_vec(n);
            if n > 1 {
                point[0] = F128::ZERO;
                point[n - 1] = F128::ONE;
            }
            let expected = super::with_merged_split_rows_override(false, || {
                super::column_bitbank_reference(&q, &point, n, k)
            });
            assert_eq!(
                super::column_bitbank_split(&q, &point, n, k),
                expected,
                "row bits={n}, column bits={k}"
            );
        }
    }

    #[test]
    fn column_bitbanks_reuse_exact_row_points_and_borrow_supplied() {
        let mut rng = Rng::new(0xB17_CA0E);
        let (n, k) = (7, 2);
        let q = rng.f128_vec(1 << (n + k));
        let mut points: Vec<Vec<F128>> = (0..4).map(|_| rng.f128_vec(1 + n + k)).collect();
        let row = points[0][1..1 + n].to_vec();
        points[1][1..1 + n].copy_from_slice(&row);
        points[3][1..1 + n].copy_from_slice(&row);
        let xs: Vec<_> = points.iter().map(Vec::as_slice).collect();
        let supplied = super::column_bitbank_split(&q, &row, n, k);
        let (banks, indices) = super::with_merged_split_rows_override(true, || {
            super::column_bitbanks_for_claims(&q, &xs, &[Some(&supplied)], n, k, false)
        });
        assert_eq!(indices, vec![0, 0, 1, 0]);
        assert_eq!(banks.len(), 2);
        assert!(matches!(&banks[0], std::borrow::Cow::Borrowed(_)));
        let (old_banks, old_indices) = super::with_merged_split_rows_override(false, || {
            super::column_bitbanks_for_claims(&q, &xs, &[], n, k, false)
        });
        assert_eq!(old_banks.len(), 4);
        for i in 0..4 {
            assert_eq!(banks[indices[i]], old_banks[old_indices[i]]);
        }
    }

    #[test]
    fn merged_split_rows_match_dense_and_bound_large_storage() {
        let mut rng = Rng::new(0xB0_0D_ED);
        for n in 0..=12 {
            let mut point = rng.f128_vec(n);
            if n > 1 {
                point[0] = F128::ZERO;
                point[n - 1] = F128::ONE;
            }
            let dense =
                super::with_merged_split_rows_override(false, || super::MergedRowEq::new(&point));
            let split =
                super::with_merged_split_rows_override(true, || super::MergedRowEq::new(&point));
            for row in 0..1 << n {
                assert_eq!(dense.at(row), split.at(row));
            }
        }
        let point = rng.f128_vec(32);
        let split = super::MergedRowEq::new(&point);
        let super::MergedRowEq::Split { lo, hi, .. } = &split else {
            panic!("large row domains must not materialize a full tensor");
        };
        assert_eq!((lo.len() + hi.len()) * std::mem::size_of::<F128>(), 2 << 20);
        for row in [0, 1, 65535, 65536, 0x8123_4567, 0xffff_ffff] {
            let expected = point.iter().enumerate().fold(F128::ONE, |acc, (bit, &r)| {
                acc * if row >> bit & 1 == 0 {
                    F128::ONE + r
                } else {
                    r
                }
            });
            assert_eq!(split.at(row), expected);
        }
    }

    #[test]
    fn block_first_split_rows_match_dense_with_scalar_groups() {
        use super::{BlockFirstClaim, block_first_phase, column_bitbank_reference};
        use crate::challenger::Challenger;
        let mut rng = Rng::new(0xB10C_F175);
        let (n, k) = (7, 3);
        let q = rng.f128_vec(1 << (n + k));
        let rows = [rng.f128_vec(n), rng.f128_vec(n)];
        let cols = [rng.f128_vec(k), vec![F128::ZERO, F128::ONE, F128::ZERO]];
        let coeffs = [rng.f128_vec(128), rng.f128_vec(128)];
        let grouped_row = rng.f128_vec(n);
        let groups = vec![(grouped_row.as_slice(), rng.f128_vec(1 << k))];
        for live_cols in [1, 3, 8] {
            let prove = |split| {
                super::with_merged_split_rows_override(split, || {
                    let banks: Vec<_> = rows
                        .iter()
                        .map(|row| column_bitbank_reference(&q, row, n, k))
                        .collect();
                    let rs: Vec<_> = (0..2)
                        .map(|i| BlockFirstClaim {
                            z_row: &rows[i],
                            z_col: &cols[i],
                            coeffs: &coeffs[i],
                            bank: &banks[i],
                        })
                        .collect();
                    let mut ch = FsChallenger::new(b"block-first-split-row-differential");
                    let (mut rounds, mut nonces, mut point) = (Vec::new(), Vec::new(), Vec::new());
                    let (qp, wp) = block_first_phase(
                        &q,
                        n,
                        k,
                        live_cols,
                        &rs,
                        &groups,
                        OpeningGrinding::disabled(),
                        &mut ch,
                        &mut rounds,
                        &mut nonces,
                        &mut point,
                        false,
                    );
                    (qp, wp, rounds, nonces, point, ch.sample_f128())
                })
            };
            assert_eq!(prove(false), prove(true));
        }
    }

    fn zhat_skip_reference(z: &[bool], m: usize, z_skip: F128, x_outer: &[F128]) -> F128 {
        const K_SKIP: usize = 6;
        let ell = 1usize << K_SKIP;
        let lambda = lagrange_weights_naive(K_SKIP, z_skip);
        let eq_outer = build_eq(x_outer);
        let mut acc = F128::ZERO;
        for i_outer in 0..(1usize << (m - K_SKIP)) {
            let base = i_outer * ell;
            let mut inner = F128::ZERO;
            for i_skip in 0..ell {
                if z[base + i_skip] {
                    inner += lambda[i_skip];
                }
            }
            acc += eq_outer[i_outer] * inner;
        }
        acc
    }

    /// End-to-end Ligerito backend roundtrip through pcs::open_batch_mixed_ligerito
    /// and verify_opening_batch_ligerito_mixed. Single ring-switched claim
    /// (no PD — PD path is task #11).
    #[test]
    #[ignore] // Heavier — ~50-100 ms; run with `cargo test pcs_ligerito_backend_roundtrip -- --ignored --nocapture`
    fn pcs_ligerito_backend_roundtrip() {
        let m = 22usize;
        let mut rng = Rng::new(0x11_6E_2170);
        let z = rng.bits(1 << m);
        let z_skip = rng.f128();
        let x_outer: Vec<F128> = (0..(m - 6)).map(|_| rng.f128()).collect();
        let rs_claim = zhat_skip_reference(&z, m, z_skip, &x_outer);

        // PcsParams MUST set log_batch_size = ligerito_initial_k for L0 reuse.
        let initial_k = 6;
        let params = PcsParams {
            m,
            log_inv_rate: 1,
            log_batch_size: initial_k,
            profile: Default::default(),
            num_lanes: None,
            merkle_hash: Default::default(),
        };
        let z_packed = pack_witness(&z, m);
        let (commitment, prover_data) = commit(&z_packed, &params);

        let log_n = m - LOG_PACKING;
        let lig_p_cfg = prover_config_for(log_n, initial_k, LigeritoProfile::Fast)
            .expect("m22 Fast prover config");
        let lig_v_cfg = verifier_config_for(log_n, initial_k, LigeritoProfile::Fast)
            .expect("m22 Fast verifier config");

        let mut ch_p = FsChallenger::new(b"flock-test-lig-v0");
        let proof = open_batch_mixed_ligerito_with_precomputed_s_hat_v_and_grinding(
            z_packed.clone(),
            &prover_data,
            &commitment,
            &[x_outer.as_slice()],
            &[],
            &[],
            &PaddingSpec::dense(m),
            &lig_p_cfg,
            OpeningGrinding::disabled(),
            &mut ch_p,
        );

        let mut ch_v = FsChallenger::new(b"flock-test-lig-v0");
        verify_opening_batch_ligerito_mixed_with_grinding(
            &commitment,
            &[rs_claim],
            &[&lagrange_weights_naive(6, z_skip)],
            &[x_outer.as_slice()],
            &[],
            &proof,
            &lig_v_cfg,
            OpeningGrinding::disabled(),
            &mut ch_v,
        )
        .unwrap_or_else(|e| panic!("ligerito verify rejected honest proof: {e:?}"));

        let mut malformed = proof.clone();
        malformed.ring_switches.clear();
        let mut ch_v = FsChallenger::new(b"flock-test-lig-v0");
        assert!(matches!(
            verify_opening_batch_ligerito_mixed_with_grinding(
                &commitment,
                &[rs_claim],
                &[&lagrange_weights_naive(6, z_skip)],
                &[x_outer.as_slice()],
                &[],
                &malformed,
                &lig_v_cfg,
                OpeningGrinding::disabled(),
                &mut ch_v,
            ),
            Err(PcsError::RingSwitch(RingSwitchError::MalformedProof))
        ));
    }

    #[test]
    fn strict_transport_grinds_over_the_johnson_level_zero_list() {
        let policy = OpeningGrinding::per_challenge_128();
        assert_eq!(policy.claim_batch_bits_for(1), 0);
        assert_eq!(policy.claim_batch_bits_for(2), 6);
    }

    #[test]
    fn merged_weighted_columns_preserve_skip64_and_bind_arbitrary128() {
        use super::{
            PcsOpenError, commit_lane_major, open_batch_merged, verify_batch_merged,
            verify_batch_merged_weighted,
        };
        use crate::{
            challenger::Challenger,
            lincheck::SkipPoint,
            pcs::ring_switch::{
                build_claim_weights_from_skip, claim_check,
                fold_1b_rows_1way_method_of_four_russians,
            },
        };

        // A partial-height dense stack exercises the production jagged path
        // and its 50-lane EqPoint inner opening, not a padded-only oracle.
        let m = 22;
        let n_log = 7;
        let row_capacity = 1usize << n_log;
        let mut heights = vec![0u64; 256];
        heights[..200].fill(127);
        let mut rng = Rng::new(0xC011_2864);
        let mut padded = vec![F128::ZERO; 1usize << (m - LOG_PACKING)];
        let mut dense = Vec::with_capacity(padded.len());
        for col in 0..200 {
            for row in 0..127 {
                let value = rng.f128();
                padded[col * row_capacity + row] = value;
                dense.push(value);
            }
        }
        dense.resize(padded.len(), F128::ZERO);
        let params = PcsParams {
            m,
            log_inv_rate: 1,
            log_batch_size: 6,
            profile: LigeritoProfile::Fast,
            num_lanes: Some(50),
            merkle_hash: Default::default(),
        };
        let (commitment, data) = commit_lane_major(&dense, &params);
        let prover_config = params.ligerito_prover_config().expect("m22 Fast config");
        let verifier_config = params.ligerito_verifier_config().expect("m22 Fast config");
        let point: Vec<_> = (0..m - 6).map(|_| rng.f128()).collect();
        let skip = SkipPoint::Phi8(rng.f128());
        let columns = build_claim_weights_from_skip(&skip.weights(6), point[0]);
        let s_hat_v = fold_1b_rows_1way_method_of_four_russians(&padded, &build_eq(&point[1..]));
        let value = claim_check(&columns, &s_hat_v);
        let grinding = OpeningGrinding::per_challenge_128();
        let mut prover = FsChallenger::new(b"merged-weighted-columns-test");
        let proof = open_batch_merged(
            dense,
            Some(&padded),
            &data,
            &commitment,
            &[&point],
            &[Some(&s_hat_v)],
            &[],
            &[],
            &PaddingSpec::dense(m),
            &heights,
            n_log,
            &prover_config,
            grinding,
            &mut prover,
        );
        let mut old = FsChallenger::new(b"merged-weighted-columns-test");
        verify_batch_merged(
            &commitment,
            &[value],
            &[skip],
            &[&point],
            &[],
            &heights,
            n_log,
            &proof,
            &verifier_config,
            grinding,
            &mut old,
        )
        .expect("legacy skip64 verifier");
        let verify = |columns: &[F128], value, proof: &super::MergedOpenProof, point: &[F128]| {
            let mut ch = FsChallenger::new(b"merged-weighted-columns-test");
            verify_batch_merged_weighted(
                &commitment,
                &[value],
                &[columns],
                &[point],
                &[],
                &heights,
                n_log,
                proof,
                &verifier_config,
                grinding,
                &mut ch,
            )
            .map(|()| ch.sample_f128())
        };
        let expected = prover.sample_f128();
        assert_eq!(old.sample_f128(), expected);
        assert_eq!(verify(&columns, value, &proof, &point), Ok(expected));

        // The identical production proof authenticates any independently
        // specified bit-linear combination of its committed slice values.
        let arbitrary: Vec<_> = (0..128).map(|_| rng.f128()).collect();
        let arbitrary_value = claim_check(&arbitrary, &s_hat_v);
        let mut dummy_point = point.clone();
        dummy_point[0] += F128::ONE;
        assert_eq!(
            verify(&arbitrary, arbitrary_value, &proof, &dummy_point),
            Ok(expected),
        );
        assert!(verify(&arbitrary, arbitrary_value + F128::ONE, &proof, &point).is_err());
        assert!(matches!(
            verify(&arbitrary[..127], arbitrary_value, &proof, &point),
            Err(PcsOpenError::RingSwitch(RingSwitchError::MalformedProof)),
        ));
        let mut bad_columns = arbitrary.clone();
        let nonzero = s_hat_v
            .iter()
            .position(|x| *x != F128::ZERO)
            .expect("nonzero slice");
        bad_columns[nonzero] += F128::ONE;
        assert!(verify(&bad_columns, arbitrary_value, &proof, &point).is_err());
        let mut bad = proof.clone();
        bad.ring_switches[0].s_hat_v.pop();
        assert!(matches!(
            verify(&arbitrary, arbitrary_value, &bad, &point),
            Err(PcsOpenError::RingSwitch(RingSwitchError::MalformedProof)),
        ));
        let mut bad = proof;
        bad.q_eval += F128::ONE;
        assert!(verify(&arbitrary, arbitrary_value, &bad, &point).is_err());
    }
}
