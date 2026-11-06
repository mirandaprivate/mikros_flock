//! Opt-in diagnostics; the uninstrumented path omits all per-round timers.
use std::time::Instant;

#[cfg(not(feature = "single-mle"))]
use binius_ip_prover::sumcheck::{
    common::SumcheckProver, multilinear_eval::multilinear_eval_prover,
};

use super::*;

pub(super) fn enabled() -> bool {
    std::env::var_os("ADDITIVE_FRI_PROFILE_STAGES").is_some()
}

pub(super) struct Stages {
    phase: &'static str,
    words: usize,
    last: Option<Instant>,
    measurements: Vec<serde_json::Value>,
}

impl Stages {
    pub(super) fn new(phase: &'static str, words: usize) -> Self {
        Self {
            phase,
            words,
            last: enabled().then(Instant::now),
            measurements: Vec::new(),
        }
    }
    pub(super) fn mark(&mut self, stage: &str) {
        if let Some(last) = self.last.as_mut() {
            let elapsed = last.elapsed().as_secs_f64() * 1e3;
            *last = Instant::now();
            self.measurements
                .push(serde_json::json!({"stage": stage, "ms": elapsed}));
        }
    }
    pub(super) fn emit(self) {
        if self.last.is_some() {
            eprintln!(
                "FRI_PROFILE {}",
                serde_json::json!({
                    "phase": self.phase, "words": self.words, "stages": self.measurements,
                })
            );
        }
    }
}

/// The non-hiding, single-oracle native schedule with timers between its public operations.
/// A degree-one MLE-check transmits only its slope; native verification is unchanged.
pub(super) fn prove_profiled<Channel: MerkleIPProverChannel<B128>>(
    message: FieldBuffer<Packed>,
    point: &[B128],
    value: B128,
    params: &FRIParams<B128>,
    mut folder: FRIFoldProver<'_, B128, Packed, Ntt, Channel::Commitment>,
    channel: &mut Channel,
) {
    let mut profile = Stages::new("open", message.len());
    #[cfg(not(feature = "single-mle"))]
    let mut sumcheck = multilinear_eval_prover(message, point, value);
    #[cfg(feature = "single-mle")]
    let mut sumcheck = crate::mle::SpecializedMle::new(message, point, value);
    for round in 0..point.len() {
        #[cfg(not(feature = "single-mle"))]
        let coeffs = sumcheck.execute().pop().expect("single claim");
        #[cfg(not(feature = "single-mle"))]
        let coeffs = coeffs.0;
        #[cfg(feature = "single-mle")]
        let coeffs = sumcheck.execute();
        profile.mark(&format!("mle_{round}"));
        assert_eq!(coeffs.len(), 2);
        channel.send_one(coeffs[1]);
        folder.execute_fold_round(channel);
        profile.mark(&format!("fri_{round}"));
        let challenge = channel.sample();
        sumcheck.fold(challenge);
        folder.receive_challenge(challenge);
    }
    folder.execute_fold_round(channel);
    profile.mark("fri_final_fold");
    let (terminal, commitment, queries) = folder.finalize();
    let indices = (0..params.n_test_queries())
        .map(|_| channel.sample_bits(params.index_bits()))
        .collect::<Vec<_>>();
    queries.prove_queries(&indices, channel);
    channel.send_committed_vector(&commitment, terminal.to_ref());
    profile.mark("queries");
    drop((queries, terminal, commitment));
    profile.mark("release_codewords");
    drop(sumcheck);
    profile.mark("release_message");
    profile.emit();
}
