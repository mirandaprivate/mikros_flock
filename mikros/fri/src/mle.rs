//! Single-column MLE-check with a stack accumulator at each evaluation leaf.
//!
//! This retains the pinned native MleStore, fused previous-fold/next-message pass,
//! split-eq reduction tree, wide multiplication, and round interpolation. Only the
//! generic per-leaf Vec<WideOutput> and Vec<Packed> become a fixed array and Packed.

use binius_field::{Field, WideMul};
use binius_iop_prover::{fri::FRIFoldProver, merkle_channel::MerkleIPProverChannel};
use binius_ip_prover::sumcheck::{
    mle_store::{EvaluationChunk, MleStore},
    multilinear_eval::MultilinearEvalEvaluator,
    round_evaluator::MleCheckRoundEvaluator,
};
use binius_math::{FieldBuffer, multilinear::eq::eq_ind_partial_eval};

use crate::{B128, Ntt, Packed};

// Match MAX_CHUNK_VARS in the pinned native round_evaluator. Keeping its exact
// split tree also gives a direct round-by-round differential test at this boundary.
const CHUNK_VARS: usize = 12;

pub(crate) struct SpecializedMle {
    store: MleStore<'static, Packed>,
    evaluator: MultilinearEvalEvaluator,
    point: Vec<B128>,
    claim: B128,
    coeffs: Option<[B128; 2]>,
    buffered_challenge: Option<B128>,
}

impl SpecializedMle {
    pub(crate) fn new(message: FieldBuffer<Packed>, point: &[B128], claim: B128) -> Self {
        assert_eq!(message.log_len(), point.len());
        // Native interpolation needs (1-alpha)^-1. The wrapper already dispatches
        // any such point to its independently tested two-half boundary prover.
        assert!(
            !point.contains(&B128::ONE),
            "alpha=ONE requires the boundary prover"
        );
        let mut store = MleStore::new(point.len());
        let column = store.push_owned(message);
        Self {
            store,
            evaluator: MultilinearEvalEvaluator::new(column),
            point: point.to_vec(),
            claim,
            coeffs: None,
            buffered_challenge: None,
        }
    }

    pub(crate) fn n_vars(&self) -> usize {
        self.store.n_vars() - usize::from(self.buffered_challenge.is_some())
    }

    #[cfg(test)]
    pub(crate) fn round_claim(&self) -> B128 {
        self.claim
    }

    pub(crate) fn execute(&mut self) -> [B128; 2] {
        let remaining = self.n_vars();
        assert!(remaining > 0);
        assert!(self.coeffs.is_none(), "execute called twice without fold");
        let chunk_vars = (remaining - 1).min(CHUNK_VARS.max(Packed::LOG_WIDTH));
        let alpha = self.point[remaining - 1];
        let eq_chunk = eq_ind_partial_eval::<Packed>(&self.point[..chunk_vars]);
        let evaluator = &self.evaluator;
        let point = &self.point;

        let map = |chunk: EvaluationChunk<'_, Packed>| {
            let mut accum = [<Packed as WideMul>::Output::default()];
            evaluator.accumulate(&chunk, eq_chunk.to_ref(), &mut accum);
            let [wide] = accum;
            Packed::reduce(wide)
        };
        let reduce = |low: Packed, high: Packed, level: usize| {
            low + Packed::broadcast(point[level]) * (high - low)
        };
        let accum = match self.buffered_challenge.take() {
            Some(challenge) => self
                .store
                .map_reduce_with_fold(chunk_vars, challenge, map, reduce),
            None => self.store.map_reduce(chunk_vars, map, reduce),
        };
        // Reuse native interpolation and lane summation, including sub-packed
        // final rounds. This allocates only the native two-coefficient Vec once
        // per round, never one Vec per leaf.
        let native = self
            .evaluator
            .interpolate(&self.store, &[accum], self.claim, alpha);
        assert_eq!(native.0.len(), 2);
        let coeffs = [native.0[0], native.0[1]];
        self.coeffs = Some(coeffs);
        coeffs
    }

    pub(crate) fn fold(&mut self, challenge: B128) {
        let [constant, slope] = self.coeffs.take().expect("fold requires execute");
        assert!(
            self.buffered_challenge.is_none(),
            "fold called twice without execute"
        );
        self.claim = constant + slope * challenge;
        self.buffered_challenge = Some(challenge);
    }

    #[cfg(test)]
    pub(crate) fn finish(mut self) -> B128 {
        assert_eq!(self.n_vars(), 0);
        if let Some(challenge) = self.buffered_challenge.take() {
            self.store.fold(challenge);
        }
        let values = self.store.final_evals();
        assert_eq!(values.len(), 1);
        values[0]
    }
}

/// The non-ZK, one-oracle opening schedule used by this crate.
pub(crate) fn prove_specialized<Channel: MerkleIPProverChannel<B128>>(
    message: FieldBuffer<Packed>,
    point: &[B128],
    value: B128,
    mut folder: FRIFoldProver<'_, B128, Packed, Ntt, Channel::Commitment>,
    channel: &mut Channel,
) {
    assert_eq!(message.log_len(), point.len());
    assert_eq!(folder.n_rounds(), point.len());
    let mut mle = SpecializedMle::new(message, point, value);
    for _ in 0..point.len() {
        let [_constant, slope] = mle.execute();
        // Exactly native RoundProof::truncate for degree one. The FRI root must
        // still follow this coefficient and precede the sampled challenge.
        channel.send_one(slope);
        folder.execute_fold_round(channel);
        let challenge = channel.sample();
        mle.fold(challenge);
        folder.receive_challenge(challenge);
    }
    // Like the native BaseFold driver, do not perform an unnecessary final store
    // fold merely to read a value the proof never transmits separately.
    folder.execute_fold_round(channel);
    folder.finish_proof(channel);
}

#[cfg(test)]
mod tests {
    use binius_iop_prover::{
        basefold::prove_mlecheck_basefold, merkle_channel::ProverMerkleTranscriptChannel,
    };
    use binius_ip_prover::sumcheck::{
        common::SumcheckProver, multilinear_eval::multilinear_eval_prover,
    };
    use binius_math::multilinear::evaluate::evaluate;

    use super::*;
    use crate::{Challenger, ProverHashSuite, Source, from_binius, to_binius};

    fn dense(index: usize, salt: u128) -> B128 {
        let value = (index as u128 + 1).wrapping_mul(0xd6e8_feb8_6659_fd93_9e37_79b9_7f4a_7c15);
        B128::new(value ^ value.rotate_left(47) ^ salt)
    }

    fn message(log_words: usize) -> FieldBuffer<Packed> {
        let mut message = FieldBuffer::<Packed>::zeros(log_words);
        for index in 0..1usize << log_words {
            message.set(
                index,
                dense(index, 0x8000_0000_0000_0000_0000_0000_0000_0087),
            );
        }
        message
    }

    fn point(log_words: usize, all_zero: bool) -> Vec<B128> {
        (0..log_words)
            .map(|index| {
                if all_zero || index % 3 == 0 {
                    B128::ZERO
                } else {
                    dense(index, 97)
                }
            })
            .collect()
    }

    #[test]
    fn specialized_mle_matches_every_native_round() {
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
                // 14 and 15 variables require recursive split-eq reductions past
                // CHUNK_VARS=12; the small cases exercise the sub-packed tail.
                for log_words in [1, 2, 5, 12, 13, 14, 15] {
                    for all_zero in [false, true] {
                        let message = message(log_words);
                        let point = point(log_words, all_zero);
                        let claim = evaluate(&message, &point);
                        let mut native = multilinear_eval_prover(message.clone(), &point, claim);
                        let mut candidate = SpecializedMle::new(message, &point, claim);
                        for round in 0..log_words {
                            assert_eq!(candidate.n_vars(), native.n_vars());
                            assert_eq!(candidate.round_claim(), native.round_claim()[0]);
                            let expected = native.execute();
                            let actual = candidate.execute();
                            assert_eq!(actual.as_slice(), expected[0].0.as_slice(), "threads={threads}, log_words={log_words}, round={round}, zero={all_zero}");
                            assert_eq!(candidate.round_claim(), native.round_claim()[0]);
                            let challenge = match round % 4 { 0 => B128::ZERO, 1 => B128::ONE, _ => dense(round, 1234) };
                            native.fold(challenge);
                            candidate.fold(challenge);
                        }
                        assert_eq!(candidate.round_claim(), native.round_claim()[0]);
                        assert_eq!(candidate.finish(), native.finish()[0]);
                    }
                }
            });
        }
    }

    #[test]
    fn specialized_mle_complete_proof_bytes_match_native() {
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    for log_words in [1, 5, 14] {
                        let message = message(log_words);
                        let native_point = point(log_words, false);
                        let true_value = evaluate(&message, &native_point);
                        let words: Vec<_> = message.iter_scalars().map(from_binius).collect();
                        let point: Vec<_> = native_point.iter().copied().map(from_binius).collect();
                        let source = Source::new(words.len(), 97).unwrap();
                        for wrong_claim in [false, true] {
                            let value = from_binius(
                                true_value + if wrong_claim { B128::ONE } else { B128::ZERO },
                            );
                            let mut proofs = Vec::new();
                            for specialized in [false, true] {
                                let (commitment, cache) = source.commit(&words).unwrap();
                                let mut transcript = crate::prover_transcript();
                                transcript.observe().write_bytes(&crate::statement(
                                    &source.key,
                                    &commitment,
                                    &point,
                                    value,
                                ));
                                let mut channel =
                                    ProverMerkleTranscriptChannel::<
                                        _,
                                        Challenger,
                                        B128,
                                        ProverHashSuite,
                                    >::new(&mut transcript);
                                let folder = FRIFoldProver::new(
                                    &source.key.params,
                                    &source.ntt,
                                    cache.codeword,
                                    cache.tree,
                                );
                                if specialized {
                                    prove_specialized(
                                        cache.message,
                                        &native_point,
                                        to_binius(value),
                                        folder,
                                        &mut channel,
                                    );
                                } else {
                                    prove_mlecheck_basefold(
                                        cache.message,
                                        &native_point,
                                        to_binius(value),
                                        None,
                                        &[],
                                        folder,
                                        &mut channel,
                                    );
                                }
                                drop(channel);
                                let proof = transcript.finalize();
                                let result = crate::verify(
                                    source.verifier_key(),
                                    &commitment,
                                    &point,
                                    value,
                                    &proof,
                                );
                                assert_eq!(result.is_ok(), !wrong_claim);
                                proofs.push(proof);
                            }
                            assert_eq!(
                                proofs[0], proofs[1],
                                "threads={threads}, log_words={log_words}, wrong={wrong_claim}"
                            );
                        }
                    }
                });
        }
    }

    #[test]
    #[should_panic(expected = "alpha=ONE requires the boundary prover")]
    fn specialized_mle_requires_existing_one_boundary_fallback() {
        SpecializedMle::new(message(1), &[B128::ONE], B128::ZERO);
    }
}
