//! Integration checks for the direct packed-claim boundary in latex/4-block.tex.

use std::collections::VecDeque;

use flock_core::zerocheck::multilinear::lagrange_weights_naive;
use flock_field::{F8, F128, phi8, test_rng::Rng};
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros::{
    Claims, Kernel, Proof, ProverScratch, check_claims, prove, prove_with_scratch, verify,
};

const KERNELS: [Kernel; 16] = [
    Kernel::Reference,
    Kernel::Lookup,
    Kernel::Native,
    Kernel::Tiled,
    Kernel::TiledBasis,
    Kernel::RegisterBasis,
    Kernel::RegisterPacked,
    Kernel::SplitAggregate,
    Kernel::HalfAggregate,
    Kernel::PairAggregate,
    Kernel::AffineAggregate,
    Kernel::JointAggregate,
    Kernel::CoefficientLookup,
    Kernel::CoefficientNative,
    Kernel::CoefficientCarryless,
    Kernel::CoefficientCarrylessNibble,
];
const CERTIFICATE_LABEL: &[u8] = b"mikros-univariate-column-and-quotient-v1";

fn witness(log_n: usize, seed: u64) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut rng = Rng::new(seed);
    let mut a = vec![0; 1 << (log_n - 3)];
    let mut b = a.clone();
    rng.fill_bytes(&mut a);
    rng.fill_bytes(&mut b);
    let c = a.iter().zip(&b).map(|(a, b)| a & b).collect();
    (a, b, c)
}

fn challenger(seed: u64) -> FsChallenger {
    let mut challenger =
        FsChallenger::with_hash(b"mikros-univariate-integration-v1", HashKind::Blake3);
    challenger.observe_bytes(&seed.to_le_bytes());
    challenger
}

fn verify_and_check(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    proof: &Proof,
    seed: u64,
) -> Result<Claims, mikros::Error> {
    let claims = verify(proof, log_n, &mut challenger(seed))?;
    check_claims(a, b, c, log_n, &claims)?;
    Ok(claims)
}

#[test]
fn univariate_kernels_have_identical_bytes_claims_and_transcripts() {
    for log_n in [7, 8, 10, 13] {
        let (a, b, c) = witness(log_n, 0xA110 + log_n as u64);
        let mut expected = None;
        for kernel in KERNELS {
            let mut prover = challenger(11);
            let (proof, claims) = prove(&a, &b, &c, log_n, kernel, &mut prover).unwrap();
            let mut verifier = challenger(11);
            assert_eq!(verify(&proof, log_n, &mut verifier).unwrap(), claims);
            check_claims(&a, &b, &c, log_n, &claims).unwrap();
            let next_challenge = prover.sample_f128();
            assert_eq!(next_challenge, verifier.sample_f128());
            assert_eq!(proof.column_claims.len(), 128);
            assert_eq!(proof.quotients.len(), 127);
            assert_eq!(proof.inner_product.rounds.len(), log_n - 7);
            assert_eq!(proof.field_elements(), 258 + 2 * (log_n - 7));
            let bytes = bincode::serialize(&proof).unwrap();
            assert_eq!(bytes.len(), 16 * proof.field_elements() + 24);
            let decoded: Proof = bincode::deserialize(&bytes).unwrap();
            assert_eq!(decoded, proof);
            let actual = (bytes, claims, next_challenge);
            if let Some(expected) = &expected {
                assert_eq!(expected, &actual, "n={log_n}, kernel={kernel:?}");
            } else {
                expected = Some(actual);
            }
        }
    }
    assert_eq!(258 + 2 * (30 - 7), 304);
}

#[test]
fn univariate_structured_rows_and_boundary_bits() {
    for log_n in [7, 10] {
        let bytes = 1 << (log_n - 3);
        for (left, right) in [(0, 0), (0, 255), (255, 255), (0x55, 0xaa), (0x55, 255)] {
            let a = vec![left; bytes];
            let b = vec![right; bytes];
            let c = vec![left & right; bytes];
            let mut expected = None;
            for kernel in KERNELS {
                let result = prove(&a, &b, &c, log_n, kernel, &mut challenger(17)).unwrap();
                verify_and_check(&a, &b, &c, log_n, &result.0, 17).unwrap();
                if let Some(expected) = &expected {
                    assert_eq!(&result, expected);
                } else {
                    expected = Some(result);
                }
            }
        }
        for bit in [0, 7, 8, 63, 64, 126, 127, 128, 255, (1 << log_n) - 1] {
            if bit >= 1 << log_n {
                continue;
            }
            let mut a = vec![0; bytes];
            a[bit / 8] = 1 << (bit % 8);
            let b = vec![255; bytes];
            for kernel in [Kernel::Lookup, Kernel::Native] {
                let (proof, _) = prove(&a, &b, &a, log_n, kernel, &mut challenger(19)).unwrap();
                verify_and_check(&a, &b, &a, log_n, &proof, 19).unwrap();
            }
        }
    }
}

#[test]
fn univariate_rejects_wrong_lengths_and_truncated_or_extended_messages() {
    let (a, b, c) = witness(10, 20);
    for log_n in [0, 6, usize::BITS as usize, usize::MAX] {
        assert!(prove(&a, &b, &c, log_n, Kernel::Lookup, &mut challenger(23)).is_err());
    }
    for column in 0..3 {
        let mut inputs = [a.clone(), b.clone(), c.clone()];
        inputs[column].pop();
        assert!(
            prove(
                &inputs[0],
                &inputs[1],
                &inputs[2],
                10,
                Kernel::Lookup,
                &mut challenger(23)
            )
            .is_err()
        );
    }
    let (proof, claims) = prove(&a, &b, &c, 10, Kernel::Lookup, &mut challenger(23)).unwrap();
    for extend in [false, true] {
        for part in 0..3 {
            let mut bad = proof.clone();
            match part {
                0 => {
                    if extend {
                        bad.column_claims.push(F128::ZERO);
                    } else {
                        bad.column_claims.pop();
                    }
                }
                1 => {
                    if extend {
                        bad.quotients.push(F128::ZERO);
                    } else {
                        bad.quotients.pop();
                    }
                }
                _ => {
                    if extend {
                        bad.inner_product.rounds.push([F128::ZERO; 2]);
                    } else {
                        bad.inner_product.rounds.pop();
                    }
                }
            }
            assert!(verify(&bad, 10, &mut challenger(23)).is_err());
        }
    }
    assert!(verify(&proof, 11, &mut challenger(23)).is_err());
    let mut bad_claims = claims;
    bad_claims.b.column_weights.pop();
    assert!(check_claims(&a, &b, &c, 10, &bad_claims).is_err());
}

#[test]
fn univariate_binds_every_message_class_and_rejects_false_c() {
    let (a, b, mut c) = witness(10, 24);
    let (proof, _) = prove(&a, &b, &c, 10, Kernel::Lookup, &mut challenger(29)).unwrap();
    let mut changes = Vec::new();
    let mut bad = proof.clone();
    bad.projected_c += F128::ONE;
    changes.push(bad);
    for index in [0, 63, 64, 127] {
        let mut bad = proof.clone();
        bad.column_claims[index] += F128::ONE;
        changes.push(bad);
    }
    for index in [0, 1, 63, 64, 125, 126] {
        let mut bad = proof.clone();
        bad.quotients[index] += F128::ONE;
        changes.push(bad);
    }
    for round in 0..proof.inner_product.rounds.len() {
        for coefficient in 0..2 {
            let mut bad = proof.clone();
            bad.inner_product.rounds[round][coefficient] += F128::ONE;
            changes.push(bad);
        }
    }
    let mut bad = proof.clone();
    bad.inner_product.final_a += F128::ONE;
    changes.push(bad);
    let mut bad = proof.clone();
    bad.inner_product.final_b += F128::ONE;
    changes.push(bad);
    for (index, bad) in changes.iter().enumerate() {
        assert!(
            verify_and_check(&a, &b, &c, 10, bad, 29).is_err(),
            "mutation={index}"
        );
    }
    assert!(verify(&proof, 10, &mut challenger(30)).is_err());
    assert!(verify(&proof, 10, &mut FsChallenger::new(b"wrong-domain")).is_err());
    for bit in [0, 63, 64, 127, 128, 1023] {
        c[bit / 8] ^= 1 << (bit % 8);
        for kernel in KERNELS {
            if let Ok((proof, _)) = prove(&a, &b, &c, 10, kernel, &mut challenger(31)) {
                assert!(verify_and_check(&a, &b, &c, 10, &proof, 31).is_err());
            }
        }
        c[bit / 8] ^= 1 << (bit % 8);
    }
}

#[test]
fn univariate_product_preserving_forgery_needs_source_authentication() {
    let (a, b, c) = witness(10, 32);
    let (mut proof, _) = prove(&a, &b, &c, 10, Kernel::Lookup, &mut challenger(37)).unwrap();
    let scale = F128::generator();
    assert!(!proof.inner_product.final_a.is_zero());
    assert!(!proof.inner_product.final_b.is_zero());
    proof.inner_product.final_a *= scale;
    proof.inner_product.final_b *= scale.inv();
    let claims = verify(&proof, 10, &mut challenger(37)).unwrap();
    assert!(check_claims(&a, &b, &c, 10, &claims).is_err());
}

#[test]
fn univariate_source_checks_all_inputs_and_preserves_weighted_b_factors() {
    let (a, b, c) = witness(10, 38);
    let (proof, claims) = prove(&a, &b, &c, 10, Kernel::Native, &mut challenger(41)).unwrap();
    verify_and_check(&a, &b, &c, 10, &proof, 41).unwrap();
    for column in 0..3 {
        let mut inputs = [a.clone(), b.clone(), c.clone()];
        for bit in [0, 63, 64, 127, 128, 1023] {
            inputs[column][bit / 8] ^= 1 << (bit % 8);
            assert!(check_claims(&inputs[0], &inputs[1], &inputs[2], 10, &claims).is_err());
            inputs[column][bit / 8] ^= 1 << (bit % 8);
        }
    }
    let mut normalized = claims;
    assert!(
        normalized
            .b
            .row_factors
            .iter()
            .any(|factor| factor[0] + factor[1] != F128::ONE)
    );
    for factor in &mut normalized.b.row_factors {
        factor[0] = F128::ONE + factor[1];
    }
    assert!(check_claims(&a, &b, &c, 10, &normalized).is_err());
}

#[test]
fn univariate_reused_scratch_does_not_retain_witness_or_shape_state() {
    let mut scratch = ProverScratch::default();
    for (run, log_n) in [7, 13, 8, 16, 7, 14].into_iter().enumerate() {
        let (a, b, c) = witness(log_n, 0xA140 + run as u64);
        let seed = 47 + run as u64;
        let kernel = KERNELS[run % KERNELS.len()];
        let mut fresh_challenger = challenger(seed);
        let expected = prove(&a, &b, &c, log_n, kernel, &mut fresh_challenger).unwrap();
        let mut reused_challenger = challenger(seed);
        let actual = prove_with_scratch(
            &a,
            &b,
            &c,
            log_n,
            kernel,
            &mut reused_challenger,
            &mut scratch,
        )
        .unwrap();
        assert_eq!(actual, expected, "run={run}, log_n={log_n}");
        assert_eq!(
            fresh_challenger.sample_f128(),
            reused_challenger.sample_f128()
        );
        verify_and_check(&a, &b, &c, log_n, &actual.0, seed).unwrap();
    }
}

#[test]
fn univariate_one_and_four_threads_produce_identical_proofs() {
    // 8192 rows cross the aggregate's 2048-row worker boundary as well as
    // the native packing and IP parallel thresholds.
    let (a, b, c) = witness(20, 0xA160);
    for kernel in [
        Kernel::Lookup,
        Kernel::Native,
        Kernel::Tiled,
        Kernel::TiledBasis,
        Kernel::RegisterBasis,
        Kernel::RegisterPacked,
        Kernel::SplitAggregate,
        Kernel::HalfAggregate,
        Kernel::PairAggregate,
        Kernel::AffineAggregate,
        Kernel::JointAggregate,
        Kernel::CoefficientLookup,
        Kernel::CoefficientNative,
        Kernel::CoefficientCarryless,
        Kernel::CoefficientCarrylessNibble,
    ] {
        let run = |threads| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    let mut ch = challenger(53);
                    let result = prove(&a, &b, &c, 20, kernel, &mut ch).unwrap();
                    verify_and_check(&a, &b, &c, 20, &result.0, 53).unwrap();
                    (result, ch.sample_f128())
                })
        };
        assert_eq!(run(1), run(4), "kernel={kernel:?}");
    }
}

#[test]
fn univariate_profiling_preserves_proofs_and_transcripts() {
    let (a, b, c) = witness(10, 0xA170);
    for kernel in KERNELS {
        let mut normal = challenger(59);
        let expected = prove(&a, &b, &c, 10, kernel, &mut normal).unwrap();
        let mut profiled = challenger(59);
        let (proof, claims, _) =
            mikros::hadamard::prove_profiled(&a, &b, &c, 10, kernel, &mut profiled).unwrap();
        assert_eq!(expected, (proof.clone(), claims));
        assert_eq!(normal.sample_f128(), profiled.sample_f128());
        verify_and_check(&a, &b, &c, 10, &proof, 59).unwrap();
    }
}

/// Override only the single challenge chosen after the certificate. Each
/// attempted draw still consumes the underlying transcript's RNG stream.
struct ScriptedChallenger {
    inner: FsChallenger,
    accepted: F128,
    pending: VecDeque<F128>,
    forced: Vec<F128>,
    certificate_count: usize,
}

impl ScriptedChallenger {
    fn new(accepted: F128) -> Self {
        Self {
            inner: challenger(61),
            accepted,
            pending: VecDeque::new(),
            forced: Vec::new(),
            certificate_count: 0,
        }
    }
}

impl Challenger for ScriptedChallenger {
    fn observe_label(&mut self, label: &[u8]) {
        self.inner.observe_label(label);
        if label == CERTIFICATE_LABEL {
            self.certificate_count += 1;
            assert_eq!(self.certificate_count, 1);
            self.pending
                .extend([phi8(F8(0)), phi8(F8(127)), self.accepted]);
        }
    }
    fn observe_bytes(&mut self, bytes: &[u8]) {
        self.inner.observe_bytes(bytes);
    }
    fn observe_f128(&mut self, value: F128) {
        self.inner.observe_f128(value);
    }
    fn observe_f128_slice(&mut self, values: &[F128]) {
        self.inner.observe_f128_slice(values);
    }
    fn sample_f128(&mut self) -> F128 {
        let sampled = self.inner.sample_f128();
        if let Some(forced) = self.pending.pop_front() {
            self.forced.push(forced);
            forced
        } else {
            sampled
        }
    }
    fn fork_from_seed(&self, seed: [F128; 2], label: &'static [u8]) -> Self {
        // The current Hadamard/IP path does not fork. A child transcript
        // starts unarmed and may arm its own script only upon observing its
        // own certificate label; it never inherits pending parent draws.
        Self {
            inner: self.inner.fork_from_seed(seed, label),
            accepted: self.accepted,
            pending: VecDeque::new(),
            forced: Vec::new(),
            certificate_count: 0,
        }
    }
    fn hash_kind(&self) -> HashKind {
        self.inner.hash_kind()
    }
}

#[test]
fn univariate_rejects_h_challenges_then_accepts_lambda_or_extension_point() {
    let (a, b, c) = witness(10, 0xA180);
    let gf256: Vec<_> = (0..=u8::MAX).map(|value| phi8(F8(value))).collect();
    let mut rng = Rng::new(0xA181);
    let outside = loop {
        let value = rng.f128();
        if !gf256.contains(&value) {
            break value;
        }
    };
    for accepted in [phi8(F8(128)), outside] {
        for kernel in KERNELS {
            let mut prover = ScriptedChallenger::new(accepted);
            let (proof, claims) = prove(&a, &b, &c, 10, kernel, &mut prover).unwrap();
            let mut verifier = ScriptedChallenger::new(accepted);
            assert_eq!(verify(&proof, 10, &mut verifier).unwrap(), claims);
            assert_eq!(prover.certificate_count, 1);
            assert_eq!(prover.forced, [phi8(F8(0)), phi8(F8(127)), accepted]);
            assert_eq!(prover.forced, verifier.forced);
            assert!(prover.pending.is_empty() && verifier.pending.is_empty());
            assert_eq!(prover.sample_f128(), verifier.sample_f128());
            // An independent public Lagrange routine pins the accepted point:
            // merely consuming the forced values later in the IP cannot pass.
            let rho = lagrange_weights_naive(7, accepted);
            assert_eq!(claims.a.column_weights, rho);
            assert_eq!(claims.b.column_weights, rho);
            check_claims(&a, &b, &c, 10, &claims).unwrap();
        }
    }
}
