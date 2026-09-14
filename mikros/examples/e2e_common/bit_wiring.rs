//! Bit expansion of the production circuit's word copy constraints.
//!
//! Index `128 * word + bit` is natural little-bit-first packed storage.
//! A word orbit becomes 128 independent bit orbits. All returned PCS claims
//! address the FULL PADDED union witness, before dense-stack compaction.
//! Construction and 0/1 field conversion are challenge-independent offline
//! work. GKR, challenge-dependent gathers and PCS opening belong online.

use flock_core::{
    challenger::Challenger,
    circuit::{CellSlot, Circuit},
    pcs::functional::TensorClaim,
    product_gkr::{
        BatchedGrinding, ProductGkrBatchedProof, prove_batched_prepared_sigma_with_grinding,
        verify_batched_with_sigma_and_grinding,
    },
    zerocheck::univariate_skip::build_eq,
};
use flock_field::F128;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

/// Offline representations of one fixed wiring witness and its public wiring.
pub struct BitWiringFixture {
    pub log_bits: usize,
    /// Natural packed bits; one F128 stores the raw bits of one cell word.
    pub packed: Vec<F128>,
    pub sigma: Vec<usize>,
    /// Baseline Flock expects one field element for each binary value.
    pub flock_values: Vec<F128>,
    /// Public sigma's field tags, prepared before the online timer.
    pub flock_sigma_tags: Vec<F128>,
}

impl BitWiringFixture {
    pub fn new(
        circuit: &Circuit,
        padded_witness: &[F128],
        public: &[F128],
    ) -> Result<Self, String> {
        let word_log = check_circuit(circuit, public)?;
        if padded_witness.len() != 1usize << word_log {
            return Err("wiring source must be the full padded union witness".into());
        }
        let cells = circuit.cells();
        let nu = cells.nu();
        let mask = (1usize << nu) - 1;
        let packed: Vec<F128> = (0..1usize << cells.mu())
            .into_par_iter()
            .map(|i| {
                let (slot, row) = (i >> nu, i & mask);
                match cells.slots()[slot] {
                    CellSlot::Gate { ty, .. } if row < circuit.counts()[ty] => {
                        padded_witness[cells.gate_word_addr(slot, row)]
                    }
                    CellSlot::Public { s } => {
                        public.get((s << nu) + row).copied().unwrap_or(F128::ZERO)
                    }
                    _ => F128::ZERO,
                }
            })
            .collect();
        let log_bits = cells.mu() + 7;
        let n = 1usize << log_bits;
        let sigma = prepare_sigma(circuit);
        let flock_sigma_tags = sigma.iter().map(|&i| F128::new(i as u64, 0)).collect();
        let flock_values = (0..n)
            .into_par_iter()
            .map(|i| {
                if bit(packed[i >> 7], i & 127) {
                    F128::ONE
                } else {
                    F128::ZERO
                }
            })
            .collect();
        Ok(Self {
            log_bits,
            packed,
            sigma,
            flock_values,
            flock_sigma_tags,
        })
    }
}

/// Verifier-key preprocessing; depends only on the public circuit.
pub fn prepare_sigma(circuit: &Circuit) -> Vec<usize> {
    (0..1usize << (circuit.cells().mu() + 7))
        .into_par_iter()
        .map(|i| (circuit.sigma()[i >> 7] << 7) | (i & 127))
        .collect()
}

/// Only values are proof messages. Query weights come from the verified PIOP
/// and the public circuit, never from a prover-supplied point.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatherProof {
    pub values: Vec<F128>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlockBitProof {
    pub gkr: ProductGkrBatchedProof,
    pub gather: GatherProof,
}

/// A complete eight-word table, bound before the fresh three-coordinate audit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CubeGatherProof {
    pub gather: GatherProof,
    pub audit_nonce: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlockBitCubeProof {
    pub gkr: ProductGkrBatchedProof,
    pub gather: CubeGatherProof,
}

/// Two PoW bits protect the cube audit's degree-at-most-three local event.
/// This is not a claim about the complete argument's combined security bound.
pub const CUBE_AUDIT_POW_BITS: u32 = 2;

/// Turn an ordinary bit-MLE endpoint into the generalized PCS query type.
pub fn mle_query(rho: &[F128], value: F128) -> Result<TensorClaim, String> {
    if rho.len() < 7 {
        return Err("bit-MLE point needs at least seven coordinates".into());
    }
    Ok(TensorClaim {
        columns: build_eq(&rho[..7]),
        rows: pairs(&rho[7..]),
        value,
    })
}

/// Gather any tensor query over wiring bits into gate-slot queries on the
/// main witness. Arbitrary/non-normalized row factors are supported.
pub fn prove_gather(
    circuit: &Circuit,
    padded_witness: &[F128],
    public: &[F128],
    source: &TensorClaim,
) -> Result<(GatherProof, Vec<TensorClaim>), String> {
    let proof = gather_values(circuit, padded_witness, public, source)?;
    let claims = verify_gather(circuit, public, source, &proof)?;
    Ok((proof, claims))
}

fn gather_values(
    circuit: &Circuit,
    padded_witness: &[F128],
    public: &[F128],
    source: &TensorClaim,
) -> Result<GatherProof, String> {
    let word_log = check_query(circuit, public, source)?;
    if padded_witness.len() != 1usize << word_log {
        return Err("gather source must be the full padded union witness".into());
    }
    let cells = circuit.cells();
    let values = (0..cells.num_gate_slots())
        .into_par_iter()
        .map(|slot| {
            let base = cells.gate_word_addr(slot, 0);
            (0..1usize << cells.nu())
                .map(|row| {
                    row_weight(&source.rows[..cells.nu()], row)
                        * word_dot(padded_witness[base + row], &source.columns)
                })
                .fold(F128::ZERO, |a, b| a + b)
        })
        .collect();
    Ok(GatherProof { values })
}

/// Verifier-only gather check. This function has NO witness parameter.
/// Returned claims must ALL be authenticated by the caller's PCS verifier.
pub fn verify_gather(
    circuit: &Circuit,
    public: &[F128],
    source: &TensorClaim,
    proof: &GatherProof,
) -> Result<Vec<TensorClaim>, String> {
    check_gather_sum(circuit, public, source, proof)?;
    let cells = circuit.cells();
    let nu = cells.nu();
    Ok((0..cells.num_gate_slots())
        .map(|slot| {
            // Fixed high address coordinates with the original low instance
            // factors, including non-normalized pairs.
            let point = cells.gate_claim_point(slot, &vec![F128::ZERO; nu]);
            let mut rows = source.rows[..nu].to_vec();
            rows.extend(pairs(&point[nu..]));
            TensorClaim {
                columns: source.columns.clone(),
                rows,
                value: proof.values[slot],
            }
        })
        .collect())
}

fn check_gather_sum(
    circuit: &Circuit,
    public: &[F128],
    source: &TensorClaim,
    proof: &GatherProof,
) -> Result<(), String> {
    check_query(circuit, public, source)?;
    let cells = circuit.cells();
    let nu = cells.nu();
    if proof.values.len() != cells.num_gate_slots() {
        return Err("wrong number of bit wiring gather values".into());
    }
    let mut sum = F128::ZERO;
    for (slot, descriptor) in cells.slots().iter().enumerate() {
        let slot_weight = row_weight(&source.rows[nu..], slot);
        match descriptor {
            CellSlot::Gate { .. } => {
                sum += slot_weight * proof.values[slot];
            }
            CellSlot::Public { s } => {
                let base = s << nu;
                for row in 0..(1usize << nu).min(public.len().saturating_sub(base)) {
                    sum += slot_weight
                        * row_weight(&source.rows[..nu], row)
                        * word_dot(public[base + row], &source.columns);
                }
            }
            CellSlot::Pad => {}
        }
    }
    if sum != source.value {
        return Err("bit wiring gather does not match PIOP endpoint".into());
    }
    Ok(())
}

/// Validate the public geometry rather than assuming a SHA-specific schema.
/// Gate slots must be a permutation of the FIRST eight complete word columns.
fn cube_layout(
    circuit: &Circuit,
    public: &[F128],
    source: &TensorClaim,
) -> Result<(usize, [usize; 8]), String> {
    let word_log = check_query(circuit, public, source)?;
    let cells = circuit.cells();
    let nu = cells.nu();
    if cells.num_gate_slots() != 8
        || nu >= usize::BITS as usize
        || word_log < nu + 3
        || cells.mu() < nu + 3
    {
        return Err("cube gather requires eight gates spanning a three-bit word cube".into());
    }
    let row_count = 1usize << nu;
    let mut slot_to_word = [0usize; 8];
    let mut seen = [false; 8];
    for slot in 0..8 {
        if !matches!(cells.slots()[slot], CellSlot::Gate { .. }) {
            return Err("cube gather requires a gate-slot prefix".into());
        }
        let base = cells.gate_word_addr(slot, 0);
        let word = base >> nu;
        if base & (row_count - 1) != 0 || word >= 8 || seen[word] {
            return Err(
                "cube gather gate addresses must cover first eight word columns exactly".into(),
            );
        }
        let point = cells.gate_claim_point(slot, &vec![F128::ZERO; nu]);
        if point.len() != word_log
            || point[..nu].iter().any(|x| !x.is_zero())
            || point[nu..].iter().enumerate().any(|(bit, &x)| {
                x != if (word >> bit) & 1 == 0 {
                    F128::ZERO
                } else {
                    F128::ONE
                }
            })
        {
            return Err("cube gather point geometry disagrees with packed word addresses".into());
        }
        seen[word] = true;
        slot_to_word[slot] = word;
    }
    Ok((word_log, slot_to_word))
}

fn bind_cube<C: Challenger>(
    circuit: &Circuit,
    source: &TensorClaim,
    gather: &GatherProof,
    word_log: usize,
    slot_to_word: &[usize; 8],
    ch: &mut C,
) -> [F128; 8] {
    let mut table = [F128::ZERO; 8];
    for (slot, &word) in slot_to_word.iter().enumerate() {
        table[word] = gather.values[slot];
    }
    ch.observe_label(b"mikros-flock-bit-gather-cube-v1");
    ch.observe_bytes(&(word_log as u64).to_le_bytes());
    ch.observe_bytes(&(circuit.cells().nu() as u64).to_le_bytes());
    ch.observe_f128_slice(&source.columns);
    ch.observe_bytes(&(source.rows.len() as u64).to_le_bytes());
    for pair in &source.rows {
        ch.observe_f128_slice(pair);
    }
    ch.observe_f128(source.value);
    for &word in slot_to_word {
        ch.observe_bytes(&(word as u64).to_le_bytes());
    }
    // Every table entry is fixed BEFORE the audit point is sampled. Binding
    // only their eventual random combination would not authenticate the table.
    ch.observe_f128_slice(&table);
    table
}

fn cube_query(
    source: &TensorClaim,
    nu: usize,
    word_log: usize,
    table: &[F128; 8],
    point: &[F128],
) -> TensorClaim {
    let value = table
        .iter()
        .zip(build_eq(point))
        .fold(F128::ZERO, |s, (&v, w)| s + v * w);
    let mut rows = source.rows[..nu].to_vec();
    rows.extend(pairs(point));
    rows.resize(word_log, [F128::ONE, F128::ZERO]);
    TensorClaim {
        columns: source.columns.clone(),
        rows,
        value,
    }
}

/// Audit the complete gathered eight-word cube and return exactly one query.
/// The caller must pass a challenger with working explicit PoW methods (e.g.
/// raw FsChallenger), and bind the circuit/public statement before this call.
/// The prior endpoint/public recombination is preserved. A false eight-value
/// table adds a nonzero polynomial of total degree at most three at this event.
pub fn prove_gather_cube<C: Challenger>(
    circuit: &Circuit,
    padded_witness: &[F128],
    public: &[F128],
    source: &TensorClaim,
    ch: &mut C,
) -> Result<(CubeGatherProof, Vec<TensorClaim>), String> {
    let (word_log, slot_to_word) = cube_layout(circuit, public, source)?;
    let gather = gather_values(circuit, padded_witness, public, source)?;
    check_gather_sum(circuit, public, source, &gather)?;
    let table = bind_cube(circuit, source, &gather, word_log, &slot_to_word, ch);
    let (audit_nonce, point) = ch.grind_pow_and_sample_f128_vec(CUBE_AUDIT_POW_BITS, 3);
    let claim = cube_query(source, circuit.cells().nu(), word_log, &table, &point);
    Ok((
        CubeGatherProof {
            gather,
            audit_nonce,
        },
        vec![claim],
    ))
}

/// Verifier replay uses public geometry, source query and table messages only.
/// The one returned query still MUST reach the shared PCS verifier.
pub fn verify_gather_cube<C: Challenger>(
    circuit: &Circuit,
    public: &[F128],
    source: &TensorClaim,
    proof: &CubeGatherProof,
    ch: &mut C,
) -> Result<Vec<TensorClaim>, String> {
    let (word_log, slot_to_word) = cube_layout(circuit, public, source)?;
    check_gather_sum(circuit, public, source, &proof.gather)?;
    let table = bind_cube(circuit, source, &proof.gather, word_log, &slot_to_word, ch);
    let point = ch
        .verify_pow_and_sample_f128_vec(proof.audit_nonce, CUBE_AUDIT_POW_BITS, 3)
        .ok_or("cube gather audit PoW rejected")?;
    Ok(vec![cube_query(
        source,
        circuit.cells().nu(),
        word_log,
        &table,
        &point,
    )])
}

/// Flock permutation baseline for the expanded bit relation. Caller binds the
/// circuit/public statement and main witness commitment before this entry.
pub fn prove_flock<C: Challenger>(
    circuit: &Circuit,
    offline: &BitWiringFixture,
    padded_witness: &[F128],
    public: &[F128],
    grinding: BatchedGrinding,
    ch: &mut C,
) -> Result<(FlockBitProof, Vec<TensorClaim>), String> {
    if offline.log_bits != circuit.cells().mu() + 7 {
        return Err("bit fixture dimension mismatch".into());
    }
    bind_bit_domain(offline.log_bits, ch);
    let (gkr, endpoint) = prove_batched_prepared_sigma_with_grinding(
        &offline.flock_values,
        &offline.flock_values,
        &offline.sigma,
        &offline.flock_sigma_tags,
        grinding,
        ch,
    );
    if endpoint.f_eval != endpoint.g_eval {
        return Err("Flock bit input evaluations disagree".into());
    }
    let source = mle_query(&endpoint.rho, endpoint.f_eval)?;
    let (gather, claims) = prove_gather(circuit, padded_witness, public, &source)?;
    Ok((FlockBitProof { gkr, gather }, claims))
}

/// Full PIOP replay with verifier-known bit sigma, followed by BOTH input
/// checks and gather reduction. No witness is read. PCS discharge is external.
pub fn verify_flock<C: Challenger>(
    circuit: &Circuit,
    bit_sigma: &[usize],
    public: &[F128],
    proof: &FlockBitProof,
    grinding: BatchedGrinding,
    ch: &mut C,
) -> Result<Vec<TensorClaim>, String> {
    let log_bits = circuit.cells().mu() + 7;
    if bit_sigma.len() != 1usize << log_bits {
        return Err("bit sigma dimension mismatch".into());
    }
    // The sigma is public verifier preprocessing; check against the circuit
    // before caching it in a VK. This helper does not accept proof-supplied sigma.
    bind_bit_domain(log_bits, ch);
    let endpoint =
        verify_batched_with_sigma_and_grinding(log_bits, &proof.gkr, bit_sigma, None, grinding, ch)
            .map_err(|e| format!("Flock bit permutation rejected: {e:?}"))?;
    if endpoint.f_eval != endpoint.g_eval {
        return Err("Flock bit g endpoint is unbound".into());
    }
    verify_gather(
        circuit,
        public,
        &mle_query(&endpoint.rho, endpoint.f_eval)?,
        &proof.gather,
    )
}

/// Flock's unchanged bit product-GKR followed by the optional cube gather.
/// The challenger must implement real explicit PoW+squeeze methods.
pub fn prove_flock_cube<C: Challenger>(
    circuit: &Circuit,
    offline: &BitWiringFixture,
    padded_witness: &[F128],
    public: &[F128],
    grinding: BatchedGrinding,
    ch: &mut C,
) -> Result<(FlockBitCubeProof, Vec<TensorClaim>), String> {
    if offline.log_bits != circuit.cells().mu() + 7 {
        return Err("bit fixture dimension mismatch".into());
    }
    bind_bit_domain(offline.log_bits, ch);
    let (gkr, endpoint) = prove_batched_prepared_sigma_with_grinding(
        &offline.flock_values,
        &offline.flock_values,
        &offline.sigma,
        &offline.flock_sigma_tags,
        grinding,
        ch,
    );
    if endpoint.f_eval != endpoint.g_eval {
        return Err("Flock bit input evaluations disagree".into());
    }
    let source = mle_query(&endpoint.rho, endpoint.f_eval)?;
    let (gather, claims) = prove_gather_cube(circuit, padded_witness, public, &source, ch)?;
    Ok((FlockBitCubeProof { gkr, gather }, claims))
}

pub fn verify_flock_cube<C: Challenger>(
    circuit: &Circuit,
    bit_sigma: &[usize],
    public: &[F128],
    proof: &FlockBitCubeProof,
    grinding: BatchedGrinding,
    ch: &mut C,
) -> Result<Vec<TensorClaim>, String> {
    let log_bits = circuit.cells().mu() + 7;
    if bit_sigma.len() != 1usize << log_bits {
        return Err("bit sigma dimension mismatch".into());
    }
    bind_bit_domain(log_bits, ch);
    let endpoint =
        verify_batched_with_sigma_and_grinding(log_bits, &proof.gkr, bit_sigma, None, grinding, ch)
            .map_err(|e| format!("Flock bit permutation rejected: {e:?}"))?;
    if endpoint.f_eval != endpoint.g_eval {
        return Err("Flock bit g endpoint is unbound".into());
    }
    verify_gather_cube(
        circuit,
        public,
        &mle_query(&endpoint.rho, endpoint.f_eval)?,
        &proof.gather,
        ch,
    )
}

fn bind_bit_domain<C: Challenger>(log_bits: usize, ch: &mut C) {
    ch.observe_label(b"mikros-flock-e2e-bit-wiring-v1");
    ch.observe_bytes(&(log_bits as u64).to_le_bytes());
}

fn check_circuit(circuit: &Circuit, public: &[F128]) -> Result<usize, String> {
    if !circuit.check_public(public) || circuit.cells().num_gate_slots() == 0 {
        return Err(
            "bit wiring expects a valid circuit public segment and at least one gate slot".into(),
        );
    }
    let nu = circuit.cells().nu();
    Ok(circuit
        .cells()
        .gate_claim_point(0, &vec![F128::ZERO; nu])
        .len())
}

fn check_query(circuit: &Circuit, public: &[F128], source: &TensorClaim) -> Result<usize, String> {
    let word_log = check_circuit(circuit, public)?;
    if source.columns.len() != 128 || source.rows.len() != circuit.cells().mu() {
        return Err("malformed bit wiring tensor query".into());
    }
    Ok(word_log)
}

fn pairs(point: &[F128]) -> Vec<[F128; 2]> {
    point.iter().map(|&r| [F128::ONE + r, r]).collect()
}

fn row_weight(pairs: &[[F128; 2]], index: usize) -> F128 {
    pairs
        .iter()
        .enumerate()
        .fold(F128::ONE, |v, (k, pair)| v * pair[(index >> k) & 1])
}

fn word_dot(word: F128, columns: &[F128]) -> F128 {
    columns
        .iter()
        .enumerate()
        .filter_map(|(i, &w)| bit(word, i).then_some(w))
        .fold(F128::ZERO, |a, b| a + b)
}

fn bit(word: F128, index: usize) -> bool {
    ((if index < 64 { word.lo } else { word.hi }) >> (index & 63)) & 1 != 0
}

#[cfg(test)]
mod tests {
    use super::{
        BitWiringFixture, CUBE_AUDIT_POW_BITS, bind_cube, check_gather_sum, cube_layout,
        prove_flock, prove_flock_cube, prove_gather, prove_gather_cube, row_weight, verify_flock,
        verify_flock_cube, verify_gather, verify_gather_cube, word_dot,
    };
    use crate::e2e_common::TreeFixture;
    use flock_core::{
        challenger::{Challenger, FsChallenger},
        circuit::Circuit,
        pcs::functional::{TensorClaim, row_weights},
        product_gkr::{
            BatchedGrinding, prove_batched_prepared_sigma_with_grinding,
            prove_batched_with_grinding,
        },
        schedule::{IoWord, Registry, TableType},
        test_rng::Rng,
    };
    use flock_field::F128;

    #[test]
    fn prepared_sigma_preserves_gkr_proof() {
        let n = 128;
        let sigma: Vec<usize> = (0..n).map(|i| (17 * i + 3) % n).collect();
        let tags: Vec<F128> = sigma.iter().map(|&i| F128::new(i as u64, 0)).collect();
        let mut rng = Rng::new(0x2345);
        let f: Vec<F128> = (0..n)
            .map(|_| {
                if rng.next_u64() & 1 != 0 {
                    F128::ONE
                } else {
                    F128::ZERO
                }
            })
            .collect();
        let g: Vec<F128> = sigma.iter().map(|&i| f[i]).collect();
        let mut original = FsChallenger::new(b"prepared-sigma-equality");
        let mut prepared = FsChallenger::new(b"prepared-sigma-equality");
        let (p0, c0) = prove_batched_with_grinding(
            &f,
            &g,
            &sigma,
            None,
            BatchedGrinding::disabled(),
            &mut original,
        );
        let (p1, c1) = prove_batched_prepared_sigma_with_grinding(
            &f,
            &g,
            &sigma,
            &tags,
            BatchedGrinding::disabled(),
            &mut prepared,
        );
        assert_eq!(p0, p1);
        assert_eq!(c0.rho, c1.rho);
        assert_eq!(
            (c0.f_eval, c0.g_eval, c0.s_sigma_eval),
            (c1.f_eval, c1.g_eval, c1.s_sigma_eval)
        );
        assert_eq!(original.sample_f128(), prepared.sample_f128());
    }

    #[test]
    fn bit_expansion_and_general_gathers() {
        let fixture = TreeFixture::new(1);
        let witness = fixture.prepare_witness();
        let bits = BitWiringFixture::new(&fixture.circuit, &witness.0, &fixture.public).unwrap();
        for i in 0..bits.sigma.len() {
            assert_eq!(bits.sigma[i] & 127, i & 127);
            assert_eq!(bits.sigma[i] >> 7, fixture.circuit.sigma()[i >> 7]);
            assert_eq!(bits.flock_values[i], bits.flock_values[bits.sigma[i]]);
        }
        let mut rng = Rng::new(0x1234);
        let mut source = TensorClaim {
            columns: (0..128)
                .map(|_| F128::new(rng.next_u64(), rng.next_u64()))
                .collect(),
            rows: (0..fixture.circuit.cells().mu())
                .map(|_| {
                    [
                        F128::new(rng.next_u64(), rng.next_u64()),
                        F128::new(rng.next_u64(), rng.next_u64()),
                    ]
                })
                .collect(),
            value: F128::ZERO,
        };
        source.value = bits
            .packed
            .iter()
            .zip(row_weights(&source.rows))
            .map(|(&word, row)| row * word_dot(word, &source.columns))
            .fold(F128::ZERO, |a, b| a + b);
        let (proof, claims) =
            prove_gather(&fixture.circuit, &witness.0, &fixture.public, &source).unwrap();
        assert_eq!(
            claims,
            verify_gather(&fixture.circuit, &fixture.public, &source, &proof).unwrap()
        );
        for claim in &claims {
            let actual: F128 = witness
                .0
                .iter()
                .zip(row_weights(&claim.rows))
                .map(|(&word, row)| row * word_dot(word, &claim.columns))
                .fold(F128::ZERO, |a, b| a + b);
            assert_eq!(actual, claim.value);
        }
        let mut tampered = proof;
        tampered.values[0] += F128::ONE;
        assert!(verify_gather(&fixture.circuit, &fixture.public, &source, &tampered).is_err());
    }

    #[test]
    fn flock_bit_piop_replay_binds_both_sides() {
        let fixture = TreeFixture::new(1);
        let witness = fixture.prepare_witness();
        let bits = BitWiringFixture::new(&fixture.circuit, &witness.0, &fixture.public).unwrap();
        let mut prover = FsChallenger::new(b"bit-wiring-test");
        let (proof, claims) = prove_flock(
            &fixture.circuit,
            &bits,
            &witness.0,
            &fixture.public,
            BatchedGrinding::disabled(),
            &mut prover,
        )
        .unwrap();
        let mut verifier = FsChallenger::new(b"bit-wiring-test");
        assert_eq!(
            claims,
            verify_flock(
                &fixture.circuit,
                &bits.sigma,
                &fixture.public,
                &proof,
                BatchedGrinding::disabled(),
                &mut verifier
            )
            .unwrap()
        );
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        let mut bad = proof;
        bad.gkr.g_eval += F128::ONE;
        assert!(
            verify_flock(
                &fixture.circuit,
                &bits.sigma,
                &fixture.public,
                &bad,
                BatchedGrinding::disabled(),
                &mut FsChallenger::new(b"bit-wiring-test")
            )
            .is_err()
        );
    }

    fn evaluate(packed: &[F128], claim: &TensorClaim) -> F128 {
        packed
            .iter()
            .zip(row_weights(&claim.rows))
            .fold(F128::ZERO, |s, (&word, row)| {
                s + row * word_dot(word, &claim.columns)
            })
    }

    fn general_source(bits: &BitWiringFixture, mu: usize) -> TensorClaim {
        let mut source = TensorClaim {
            columns: (1..=128).map(|j| F128::new(j, 0)).collect(),
            rows: (0..mu)
                .map(|j| {
                    [
                        F128::new(2 * j as u64 + 2, 0),
                        F128::new(2 * j as u64 + 3, 0),
                    ]
                })
                .collect(),
            value: F128::ZERO,
        };
        // Exercise both non-normalized and nonzero zero-sum instance factors.
        source.rows[0] = [F128::new(7, 0); 2];
        source.rows[1] = [F128::new(3, 0), F128::new(7, 0)];
        source.value = evaluate(&bits.packed, &source);
        source
    }

    #[test]
    fn cube_gather_general_tensor_padding_public_and_transcript() {
        let fixture = TreeFixture::new(1);
        let witness = fixture.prepare_witness();
        let bits = BitWiringFixture::new(&fixture.circuit, &witness.0, &fixture.public).unwrap();
        let source = general_source(&bits, fixture.circuit.cells().mu());
        let mut pch = FsChallenger::new(b"cube-gather-general");
        let (proof, claims) = prove_gather_cube(
            &fixture.circuit,
            &witness.0,
            &fixture.public,
            &source,
            &mut pch,
        )
        .unwrap();
        assert_eq!(claims.len(), 1);
        assert_eq!(evaluate(&witness.0, &claims[0]), claims[0].value);
        let mut vch = FsChallenger::new(b"cube-gather-general");
        assert_eq!(
            claims,
            verify_gather_cube(&fixture.circuit, &fixture.public, &source, &proof, &mut vch)
                .unwrap()
        );
        assert_eq!(pch.sample_f128(), vch.sample_f128());
        let (legacy, legacy_claims) =
            prove_gather(&fixture.circuit, &witness.0, &fixture.public, &source).unwrap();
        assert_eq!(proof.gather, legacy);
        assert_eq!(legacy_claims.len(), 8);
        assert!(
            claims[0].rows[fixture.nu + 3..]
                .iter()
                .all(|p| *p == [F128::ONE, F128::ZERO])
        );
        assert_eq!(&claims[0].rows[..fixture.nu], &source.rows[..fixture.nu]);
        let mut bad_public = fixture.public.clone();
        bad_public[0] += F128::ONE;
        assert!(
            verify_gather_cube(
                &fixture.circuit,
                &bad_public,
                &source,
                &proof,
                &mut FsChallenger::new(b"cube-gather-general")
            )
            .is_err()
        );
        for extra in [false, true] {
            let mut bad = proof.clone();
            if extra {
                bad.gather.values.push(F128::ZERO);
            } else {
                bad.gather.values.pop();
            }
            assert!(
                verify_gather_cube(
                    &fixture.circuit,
                    &fixture.public,
                    &source,
                    &bad,
                    &mut FsChallenger::new(b"cube-gather-general")
                )
                .is_err()
            );
        }
        let (word_log, map) = cube_layout(&fixture.circuit, &fixture.public, &source).unwrap();
        assert_eq!(map, [0, 1, 4, 5, 6, 7, 2, 3]);
        let invalid = (0..1000)
            .find(|&nonce| {
                let mut ch = FsChallenger::new(b"cube-gather-general");
                bind_cube(
                    &fixture.circuit,
                    &source,
                    &proof.gather,
                    word_log,
                    &map,
                    &mut ch,
                );
                ch.verify_pow_and_sample_f128_vec(nonce, CUBE_AUDIT_POW_BITS, 3)
                    .is_none()
            })
            .expect("an invalid two-bit PoW nonce");
        let mut bad = proof;
        bad.audit_nonce = invalid;
        assert!(
            verify_gather_cube(
                &fixture.circuit,
                &fixture.public,
                &source,
                &bad,
                &mut FsChallenger::new(b"cube-gather-general")
            )
            .is_err()
        );
    }

    #[test]
    fn cube_balanced_false_table_survives_old_sum_but_fails_source_query() {
        let fixture = TreeFixture::new(1);
        let witness = fixture.prepare_witness();
        let bits = BitWiringFixture::new(&fixture.circuit, &witness.0, &fixture.public).unwrap();
        let source = general_source(&bits, fixture.circuit.cells().mu());
        let (mut forged, honest_claims) = prove_gather_cube(
            &fixture.circuit,
            &witness.0,
            &fixture.public,
            &source,
            &mut FsChallenger::new(b"cube-balanced"),
        )
        .unwrap();
        let alpha0 = row_weight(&source.rows[fixture.nu..], 0);
        let alpha1 = row_weight(&source.rows[fixture.nu..], 1);
        assert!(!alpha0.is_zero() && !alpha1.is_zero());
        forged.gather.values[0] += alpha1;
        forged.gather.values[1] += alpha0;
        check_gather_sum(&fixture.circuit, &fixture.public, &source, &forged.gather).unwrap();
        // Give the adversary a valid new nonce for its already-fixed false
        // table. Rejection must rely on the downstream PCS obligation, not
        // merely the changed transcript invalidating an old PoW nonce.
        let (word_log, map) = cube_layout(&fixture.circuit, &fixture.public, &source).unwrap();
        let mut adversary = FsChallenger::new(b"cube-balanced");
        bind_cube(
            &fixture.circuit,
            &source,
            &forged.gather,
            word_log,
            &map,
            &mut adversary,
        );
        forged.audit_nonce = adversary
            .grind_pow_and_sample_f128_vec(CUBE_AUDIT_POW_BITS, 3)
            .0;
        let claims = verify_gather_cube(
            &fixture.circuit,
            &fixture.public,
            &source,
            &forged,
            &mut FsChallenger::new(b"cube-balanced"),
        )
        .unwrap();
        assert_ne!(claims, honest_claims);
        assert_ne!(evaluate(&witness.0, &claims[0]), claims[0].value);
    }

    #[test]
    fn cube_layout_rejects_non_cube_geometry_and_bad_source_shape() {
        let fixture = TreeFixture::new(1);
        let witness = fixture.prepare_witness();
        let bits = BitWiringFixture::new(&fixture.circuit, &witness.0, &fixture.public).unwrap();
        let source = general_source(&bits, fixture.circuit.cells().mu());
        for columns in [vec![0, 1, 2, 3, 4, 5, 6], vec![0, 1, 2, 3, 4, 5, 6, 8]] {
            let registry = Registry::new(
                vec![
                    TableType::from_block_r1cs(&fixture.r1cs)
                        .with_io_schema(columns.into_iter().map(IoWord::input).collect()),
                ],
                fixture.nu,
            );
            let circuit = Circuit::new(
                &registry,
                vec![fixture.compressions.len()],
                fixture.public.len(),
                vec![],
            )
            .unwrap();
            assert!(cube_layout(&circuit, &fixture.public, &source).is_err());
        }
        // Eight slots in two table prefixes do not occupy the first cube.
        let ty = TableType::from_block_r1cs(&fixture.r1cs)
            .with_io_schema((0..4).map(IoWord::input).collect());
        let registry = Registry::new(vec![ty.clone(), ty], fixture.nu);
        let circuit = Circuit::new(&registry, vec![3, 3], fixture.public.len(), vec![]).unwrap();
        assert!(cube_layout(&circuit, &fixture.public, &source).is_err());
        let mut malformed = source.clone();
        malformed.rows.pop();
        assert!(cube_layout(&fixture.circuit, &fixture.public, &malformed).is_err());
        let mut malformed = source;
        malformed.columns.pop();
        assert!(cube_layout(&fixture.circuit, &fixture.public, &malformed).is_err());
    }

    #[test]
    fn flock_cube_preserves_gkr_and_replays_one_query() {
        let fixture = TreeFixture::new(1);
        let witness = fixture.prepare_witness();
        let bits = BitWiringFixture::new(&fixture.circuit, &witness.0, &fixture.public).unwrap();
        let mut pch = FsChallenger::new(b"flock-cube");
        let (proof, claims) = prove_flock_cube(
            &fixture.circuit,
            &bits,
            &witness.0,
            &fixture.public,
            BatchedGrinding::disabled(),
            &mut pch,
        )
        .unwrap();
        assert_eq!(claims.len(), 1);
        assert_eq!(evaluate(&witness.0, &claims[0]), claims[0].value);
        let mut vch = FsChallenger::new(b"flock-cube");
        assert_eq!(
            claims,
            verify_flock_cube(
                &fixture.circuit,
                &bits.sigma,
                &fixture.public,
                &proof,
                BatchedGrinding::disabled(),
                &mut vch
            )
            .unwrap()
        );
        assert_eq!(pch.sample_f128(), vch.sample_f128());
        let (legacy, _) = prove_flock(
            &fixture.circuit,
            &bits,
            &witness.0,
            &fixture.public,
            BatchedGrinding::disabled(),
            &mut FsChallenger::new(b"flock-cube"),
        )
        .unwrap();
        assert_eq!(proof.gkr, legacy.gkr);
        assert_eq!(proof.gather.gather, legacy.gather);
    }
}
