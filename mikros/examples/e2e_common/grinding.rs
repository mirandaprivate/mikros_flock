//! Matched experimental Fiat--Shamir policy for the two PIOP branches.
//! Boolean scalar/vector challenges use an 8-bit PoW filter; permutation uses
//! a degree-dependent initial filter and two bits for its cubic tail. This
//! does not relabel the unground Mikros microbenchmark as Flock's strict profile.
//! PCS uses its own unchanged embedded Fast profile outside this wrapper.

use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use std::time::Instant;

pub const PIOP_GRIND_BITS: u32 = 8;

pub struct GrindingChallenger {
    inner: FsChallenger,
    nonces: Vec<u64>,
    verifier: bool,
    cursor: usize,
    valid: bool,
    bits: u32,
    first_bits: Option<u32>,
    next_bits: Option<u32>,
    packed_column_bits: Option<u32>,
    grinding_ms: f64,
    initial_fingerprint_grinding_ms: f64,
}

impl GrindingChallenger {
    pub fn prover(mut inner: FsChallenger) -> Self {
        inner.observe_label(b"e2e-common-piop-pow8-v1");
        Self {
            inner,
            nonces: vec![],
            verifier: false,
            cursor: 0,
            valid: true,
            bits: PIOP_GRIND_BITS,
            first_bits: None,
            next_bits: None,
            packed_column_bits: None,
            grinding_ms: 0.0,
            initial_fingerprint_grinding_ms: 0.0,
        }
    }
    pub fn verifier(mut inner: FsChallenger, nonces: &[u64]) -> Self {
        inner.observe_label(b"e2e-common-piop-pow8-v1");
        Self {
            inner,
            nonces: nonces.to_vec(),
            verifier: true,
            cursor: 0,
            valid: true,
            bits: PIOP_GRIND_BITS,
            first_bits: None,
            next_bits: None,
            packed_column_bits: None,
            grinding_ms: 0.0,
            initial_fingerprint_grinding_ms: 0.0,
        }
    }
    /// Mikros permutation: degree N-1 fingerprint, then cubic layer rounds.
    /// The first event uses log2(N) bits; subsequent events use two bits.
    /// This is a per-event work-factor policy, not a total 128-bit theorem.
    pub fn permutation(mut self, log_bits: usize) -> Self {
        self.bits = 2;
        self.first_bits = Some(log_bits as u32);
        self
    }
    /// Same fingerprint and cubic policy, with a degree-dependent filter on
    /// every labeled packed-column challenge (including rejected domain points).
    pub fn packed_permutation(mut self, log_bits: usize, kappa: usize) -> Self {
        assert!(kappa >= 2 && kappa.is_power_of_two());
        self = self.permutation(log_bits);
        let degree = 2 * kappa - 2;
        self.packed_column_bits = Some(usize::BITS - (degree - 1).leading_zeros());
        self
    }
    pub fn finish(self) -> Result<(FsChallenger, Vec<u64>), String> {
        if !self.valid || (self.verifier && self.cursor != self.nonces.len()) {
            return Err("PIOP challenge grinding rejected".into());
        }
        Ok((self.inner, self.nonces))
    }
    /// Prover wall time spent inside this wrapper's PoW-and-squeeze calls.
    /// Cube-gather and PCS grinding happen outside this wrapper.
    pub fn grinding_ms(&self) -> f64 {
        self.grinding_ms
    }
    pub fn initial_fingerprint_grinding_ms(&self) -> f64 {
        self.initial_fingerprint_grinding_ms
    }
    fn next_nonce(&mut self) -> u64 {
        let nonce = self.nonces.get(self.cursor).copied();
        self.cursor += 1;
        if nonce.is_none() {
            self.valid = false;
        }
        nonce.unwrap_or(0)
    }
}

impl Challenger for GrindingChallenger {
    fn observe_label(&mut self, label: &[u8]) {
        self.inner.observe_label(label);
        if label == b"mikros-packed-permutation-column-challenge-v1" {
            self.next_bits = self.packed_column_bits;
        }
    }
    fn observe_bytes(&mut self, bytes: &[u8]) {
        self.inner.observe_bytes(bytes);
    }
    fn observe_f128(&mut self, x: F128) {
        self.inner.observe_f128(x);
    }
    fn observe_f128_slice(&mut self, xs: &[F128]) {
        self.inner.observe_f128_slice(xs);
    }
    fn sample_f128(&mut self) -> F128 {
        let initial_fingerprint = self.first_bits.is_some();
        let bits = self
            .first_bits
            .take()
            .or_else(|| self.next_bits.take())
            .unwrap_or(self.bits);
        if self.verifier {
            let nonce = self.next_nonce();
            if let Some(r) = self.inner.verify_pow_and_sample_f128(nonce, bits) {
                return r;
            }
            self.valid = false;
            // A trait sample cannot return Result. Continue with a real squeeze
            // (not a constant that could trap rejection sampling); finish rejects.
            self.inner.sample_f128()
        } else {
            let start = Instant::now();
            let (nonce, r) = self.inner.grind_pow_and_sample_f128(bits);
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            self.grinding_ms += elapsed;
            if initial_fingerprint {
                self.initial_fingerprint_grinding_ms += elapsed;
            }
            self.nonces.push(nonce);
            r
        }
    }
    fn sample_f128_vec(&mut self, n: usize) -> Vec<F128> {
        let initial_fingerprint = self.first_bits.is_some();
        let bits = self
            .first_bits
            .take()
            .or_else(|| self.next_bits.take())
            .unwrap_or(self.bits);
        if self.verifier {
            let nonce = self.next_nonce();
            if let Some(r) = self.inner.verify_pow_and_sample_f128_vec(nonce, bits, n) {
                return r;
            }
            self.valid = false;
            self.inner.sample_f128_vec(n)
        } else {
            let start = Instant::now();
            let (nonce, r) = self.inner.grind_pow_and_sample_f128_vec(bits, n);
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            self.grinding_ms += elapsed;
            if initial_fingerprint {
                self.initial_fingerprint_grinding_ms += elapsed;
            }
            self.nonces.push(nonce);
            r
        }
    }
    fn hash_kind(&self) -> HashKind {
        self.inner.hash_kind()
    }
    fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
        panic!("fork the E2E parent before wrapping its sequential PIOP branch")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_and_tamper() {
        let run = |ch: &mut GrindingChallenger| {
            ch.observe_label(b"a");
            let a = ch.sample_f128();
            ch.observe_f128(a);
            (a, ch.sample_f128_vec(5))
        };
        let mut p = GrindingChallenger::prover(FsChallenger::new(b"pow-test"));
        let x = run(&mut p);
        let (mut pc, ns) = p.finish().unwrap();
        let mut v = GrindingChallenger::verifier(FsChallenger::new(b"pow-test"), &ns);
        assert_eq!(x, run(&mut v));
        let (mut vc, _) = v.finish().unwrap();
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        let mut v = GrindingChallenger::verifier(FsChallenger::new(b"pow-test"), &ns[..1]);
        run(&mut v);
        assert!(v.finish().is_err());
        let mut extra = ns;
        extra.push(0);
        let mut v = GrindingChallenger::verifier(FsChallenger::new(b"pow-test"), &extra);
        run(&mut v);
        assert!(v.finish().is_err());
    }

    #[test]
    fn packed_column_event_replays_and_rejects_wrong_label_and_nonce_count() {
        let run = |c: &mut GrindingChallenger, packed_label: bool| {
            let z = c.sample_f128();
            c.observe_f128(z);
            c.observe_label(if packed_label {
                b"mikros-packed-permutation-column-challenge-v1"
            } else {
                b"changed-packed-column-challenge"
            });
            let x = c.sample_f128();
            c.observe_f128(x);
            [z, x, c.sample_f128()]
        };
        let mut p = GrindingChallenger::prover(FsChallenger::new(b"packed-pow-test"))
            .packed_permutation(4, 128);
        assert_eq!(p.packed_column_bits, Some(8));
        assert_eq!(
            GrindingChallenger::prover(FsChallenger::new(b"packed-pow-kappa4"))
                .packed_permutation(4, 4)
                .packed_column_bits,
            Some(3)
        );
        let values = run(&mut p, true);
        let (mut pc, nonces) = p.finish().unwrap();
        let mut v = GrindingChallenger::verifier(FsChallenger::new(b"packed-pow-test"), &nonces)
            .packed_permutation(4, 128);
        assert_eq!(values, run(&mut v, true));
        let (mut vc, _) = v.finish().unwrap();
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        let mut v = GrindingChallenger::verifier(FsChallenger::new(b"packed-pow-test"), &nonces)
            .packed_permutation(4, 128);
        let changed = run(&mut v, false);
        assert!(changed != values || v.finish().is_err());
        let mut truncated = nonces.clone();
        truncated.pop();
        let mut v = GrindingChallenger::verifier(FsChallenger::new(b"packed-pow-test"), &truncated)
            .packed_permutation(4, 128);
        run(&mut v, true);
        assert!(v.finish().is_err());
        let mut extra = nonces;
        extra.push(0);
        let mut v = GrindingChallenger::verifier(FsChallenger::new(b"packed-pow-test"), &extra)
            .packed_permutation(4, 128);
        run(&mut v, true);
        assert!(v.finish().is_err());
    }
}
