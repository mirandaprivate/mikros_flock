//! Laurent packed IP checks at the deferred packed-opening boundary.

use std::collections::VecDeque;

use flock_field::{F128, test_rng::Rng};
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros::packed_ip::{
    self, Proof, ProverScratch, check_claims, prove, prove_with_scratch, verify,
};

fn challenger(seed: u64) -> FsChallenger {
    let mut ch = FsChallenger::with_hash(b"mikros-packed-ip-tests-v1", HashKind::Blake3);
    // In composition these bytes are the fixed statement and commitment roots.
    ch.observe_bytes(&seed.to_le_bytes());
    ch
}

fn witness(log_n: usize, seed: u64) -> (Vec<u8>, Vec<u8>, F128) {
    let mut rng = Rng::new(seed);
    let mut a = vec![0; 1 << (log_n - 3)];
    let mut b = a.clone();
    rng.fill_bytes(&mut a);
    rng.fill_bytes(&mut b);
    let claim = a
        .iter()
        .zip(&b)
        .fold(0, |parity, (a, b)| parity ^ (a & b).count_ones())
        & 1;
    (a, b, F128::new(claim as u64, 0))
}

fn replay(
    proof: &Proof,
    a: &[u8],
    b: &[u8],
    log_n: usize,
    claim: F128,
    seed: u64,
) -> Result<(), mikros::Error> {
    let claims = verify(proof, log_n, claim, &mut challenger(seed))?;
    check_claims(a, b, log_n, &claims)
}

#[test]
fn packed_ip_roundtrips_real_monomial_queries_and_transcripts() {
    for log_n in [7, 8, 10, 14, 17] {
        let (a, b, claim) = witness(log_n, 0xC011 + log_n as u64);
        let mut p = challenger(11);
        let (proof, claims) = prove(&a, &b, log_n, claim, &mut p).unwrap();
        let mut v = challenger(11);
        assert_eq!(verify(&proof, log_n, claim, &mut v).unwrap(), claims);
        assert_eq!(p.sample_f128(), v.sample_f128());
        check_claims(&a, &b, log_n, &claims).unwrap();
        assert_eq!(proof.inner_product.rounds.len(), log_n - 7);
        assert_eq!(proof.field_elements(), 2 * (log_n - 7) + 2);
        let bytes = bincode::serialize(&proof).unwrap();
        assert_eq!(bytes.len(), proof.payload_bytes() + 8);
        assert_eq!(bincode::deserialize::<Proof>(&bytes).unwrap(), proof);
        assert_eq!(claims.a.column_weights[0], F128::ONE);
        assert_eq!(claims.b.column_weights[0], F128::ONE);
        for j in 0..128 {
            assert_eq!(
                claims.a.column_weights[j] * claims.b.column_weights[j],
                F128::ONE
            );
        }
        for j in 1..128 {
            assert_eq!(
                claims.a.column_weights[j],
                claims.a.column_weights[j - 1] * claims.a.column_weights[1]
            );
        }
        assert_eq!(claims.a.row_factors, claims.b.row_factors);
        // An eq-table sums to 1. General monomial powers do not; this catches
        // the erroneous replacement with an MLE point at powers of x.
        assert_ne!(
            claims
                .a
                .column_weights
                .iter()
                .copied()
                .fold(F128::ZERO, |a, b| a + b),
            F128::ONE
        );
    }
}

#[test]
fn packed_ip_handles_zero_one_and_extreme_laurent_exponents() {
    for log_n in [7, 10] {
        let len = 1 << (log_n - 3);
        for (left, right) in [(0, 0), (0, 255), (255, 255), (0x55, 0xaa), (0x55, 255)] {
            let a = vec![left; len];
            let b = vec![right; len];
            let (proof, _) = prove(&a, &b, log_n, F128::ZERO, &mut challenger(17)).unwrap();
            replay(&proof, &a, &b, log_n, F128::ZERO, 17).unwrap();
        }
        for (left, right) in [(0, 127), (127, 0), (63, 63), (0, 64), (64, 0)] {
            let mut a = vec![0; len];
            let mut b = vec![0; len];
            a[left / 8] = 1 << (left % 8);
            b[right / 8] = 1 << (right % 8);
            let claim = if left == right { F128::ONE } else { F128::ZERO };
            let (proof, _) = prove(&a, &b, log_n, claim, &mut challenger(19)).unwrap();
            replay(&proof, &a, &b, log_n, claim, 19).unwrap();
        }
    }
}

#[test]
fn packed_ip_rejects_bad_claim_dimensions_padding_and_messages() {
    let (a, b, claim) = witness(10, 0xC022);
    let (proof, _) = prove(&a, &b, 10, claim, &mut challenger(23)).unwrap();
    assert!(prove(&a, &b, 10, claim + F128::ONE, &mut challenger(23)).is_err());
    assert!(verify(&proof, 10, claim + F128::ONE, &mut challenger(23)).is_err());
    assert!(verify(&proof, 10, F128::generator(), &mut challenger(23)).is_err());
    assert!(prove(&a, &b, 10, F128::generator(), &mut challenger(23)).is_err());
    assert!(verify(&proof, 10, claim, &mut challenger(24)).is_err());
    for log_n in [0, 6, 11, usize::BITS as usize, usize::MAX] {
        assert!(verify(&proof, log_n, claim, &mut challenger(23)).is_err());
        assert!(prove(&a, &b, log_n, claim, &mut challenger(23)).is_err());
    }
    assert!(prove(&a[..a.len() - 1], &b, 10, claim, &mut challenger(23)).is_err());
    assert!(prove(&a, &b[..b.len() - 1], 10, claim, &mut challenger(23)).is_err());
    for side in 0..2 {
        let mut bad = proof.clone();
        if side == 0 {
            bad.h_plus[15] |= 0x80;
        } else {
            bad.h_minus[15] |= 0x80;
        }
        assert!(verify(&bad, 10, claim, &mut challenger(23)).is_err());
        for coefficient in [0, 63, 64, 126] {
            let mut bad = proof.clone();
            let message = if side == 0 {
                &mut bad.h_plus
            } else {
                &mut bad.h_minus
            };
            message[coefficient / 8] ^= 1 << (coefficient % 8);
            assert!(replay(&bad, &a, &b, 10, claim, 23).is_err());
        }
    }
    let mut bad = proof.clone();
    bad.inner_product.rounds.pop();
    assert!(verify(&bad, 10, claim, &mut challenger(23)).is_err());
    let mut bad = proof.clone();
    bad.inner_product.rounds.push([F128::ZERO; 2]);
    assert!(verify(&bad, 10, claim, &mut challenger(23)).is_err());
    for column in 0..2 {
        let mut bad = proof.clone();
        bad.inner_product.rounds[0][column] += F128::ONE;
        assert!(replay(&bad, &a, &b, 10, claim, 23).is_err());
        let mut bad = proof.clone();
        if column == 0 {
            bad.inner_product.final_a += F128::ONE;
        } else {
            bad.inner_product.final_b += F128::ONE;
        }
        assert!(replay(&bad, &a, &b, 10, claim, 23).is_err());
    }
}

#[test]
fn packed_ip_authenticates_both_sources_and_reused_scratch() {
    let mut scratch = ProverScratch::default();
    for (run, log_n) in [7, 17, 8, 14, 7].into_iter().enumerate() {
        let (a, b, claim) = witness(log_n, 0xC033 + run as u64);
        let mut fresh = challenger(29);
        let expected = prove(&a, &b, log_n, claim, &mut fresh).unwrap();
        let mut reused = challenger(29);
        let actual = prove_with_scratch(&a, &b, log_n, claim, &mut reused, &mut scratch).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(fresh.sample_f128(), reused.sample_f128());
        replay(&actual.0, &a, &b, log_n, claim, 29).unwrap();
        for source in 0..2 {
            let mut inputs = [a.clone(), b.clone()];
            inputs[source][0] ^= 1;
            assert!(check_claims(&inputs[0], &inputs[1], log_n, &actual.1).is_err());
        }
        let mut bad = actual.1.clone();
        bad.b.column_weights.pop();
        assert!(check_claims(&a, &b, log_n, &bad).is_err());
        let mut bad = actual.1;
        bad.a.row_factors.push([F128::ONE, F128::ZERO]);
        assert!(check_claims(&a, &b, log_n, &bad).is_err());
    }
}

#[test]
fn packed_ip_one_and_four_threads_match() {
    let (a, b, claim) = witness(20, 0xC044);
    let run = |threads| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| {
                let mut ch = challenger(31);
                let result = prove(&a, &b, 20, claim, &mut ch).unwrap();
                (result, ch.sample_f128())
            })
    };
    let one = run(1);
    assert_eq!(one, run(4));
    replay(&one.0.0, &a, &b, 20, claim, 31).unwrap();
}

struct NonzeroChallenge {
    inner: FsChallenger,
    pending: VecDeque<F128>,
    forced: Vec<F128>,
    coefficient_bytes: usize,
    armed: bool,
    accepted: F128,
}
impl NonzeroChallenge {
    fn new(accepted: F128) -> Self {
        Self {
            inner: challenger(37),
            pending: VecDeque::new(),
            forced: Vec::new(),
            coefficient_bytes: 0,
            armed: false,
            accepted,
        }
    }
}
impl Challenger for NonzeroChallenge {
    fn observe_label(&mut self, label: &[u8]) {
        self.inner.observe_label(label);
        if label == b"mikros-binary-packed-ip-laurent-coefficients-v1" {
            self.armed = true;
            self.pending.extend([F128::ZERO, F128::ZERO, self.accepted]);
        }
    }
    fn observe_bytes(&mut self, bytes: &[u8]) {
        self.inner.observe_bytes(bytes);
        if self.armed {
            self.coefficient_bytes += bytes.len();
        }
    }
    fn observe_f128(&mut self, value: F128) {
        self.inner.observe_f128(value);
    }
    fn sample_f128(&mut self) -> F128 {
        assert!(self.armed, "sampled before coefficient message");
        assert_eq!(self.coefficient_bytes, 32);
        let value = self.inner.sample_f128();
        if let Some(forced) = self.pending.pop_front() {
            self.forced.push(forced);
            forced
        } else {
            value
        }
    }
    fn fork_from_seed(&self, seed: [F128; 2], label: &'static [u8]) -> Self {
        let mut result = Self::new(self.accepted);
        result.inner = self.inner.fork_from_seed(seed, label);
        result
    }
    fn hash_kind(&self) -> HashKind {
        self.inner.hash_kind()
    }
}

#[test]
fn packed_ip_binds_coefficients_before_nonzero_challenge_and_accepts_one() {
    for accepted in [F128::ONE, F128::generator()] {
        let (a, b, claim) = witness(10, 0xC055);
        let mut p = NonzeroChallenge::new(accepted);
        let (proof, claims) = prove(&a, &b, 10, claim, &mut p).unwrap();
        let mut v = NonzeroChallenge::new(accepted);
        assert_eq!(verify(&proof, 10, claim, &mut v).unwrap(), claims);
        assert_eq!(p.forced, [F128::ZERO, F128::ZERO, accepted]);
        assert_eq!(p.forced, v.forced);
        assert_eq!(p.sample_f128(), v.sample_f128());
        packed_ip::check_claims(&a, &b, 10, &claims).unwrap();
    }
}
