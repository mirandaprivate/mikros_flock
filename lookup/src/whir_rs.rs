//! Reed-Solomon evaluation on an additive F128 domain. No ±x butterflies,
//! multiplicative 2-power roots, coefficient reinterpretation, or extra field.
use crate::whir_field::Ghash;
use ark_ff::{AdditiveGroup, Field};
use std::sync::{Arc, OnceLock};
use whir::{
    algebra::ntt::{Messages, NTT, ReedSolomon},
    buffer::{Buffer, BufferOps},
};

const MAX_LOG_DOMAIN: usize = 21;
#[derive(Debug)]
pub(crate) struct AdditiveRs {
    vanishing: Vec<Vec<Ghash>>,
}
impl AdditiveRs {
    fn new() -> Self {
        let mut vanishing = vec![vec![Ghash::ONE]];
        for s in 0..MAX_LOG_DOMAIN {
            let old = &vanishing[s];
            let v = linearized(old, Ghash(1u128 << s));
            assert_ne!(v, Ghash::ZERO, "additive-domain basis must be independent");
            let mut next = vec![Ghash::ZERO; old.len() + 1];
            for (i, &c) in old.iter().enumerate() {
                next[i] += v * c;
                next[i + 1] += c.square();
            }
            vanishing.push(next);
        }
        Self { vanishing }
    }
    // Remainder modulo V_s(X)+V_s(offset), a monic degree-2^s polynomial
    // vanishing on offset + span(1,X,...,X^(s-1)) in the GHASH basis.
    fn remainder(&self, coefficients: &[Ghash], s: usize, offset: Ghash) -> Vec<Ghash> {
        let degree = 1usize << s;
        if coefficients.len() <= degree {
            return coefficients.to_vec();
        }
        let mut out = coefficients.to_vec();
        let constant = linearized(&self.vanishing[s], offset);
        for i in (degree..out.len()).rev() {
            let leading = out[i];
            out[i] = Ghash::ZERO;
            if leading == Ghash::ZERO {
                continue;
            }
            out[i - degree] += leading * constant;
            for j in 0..s {
                out[i - degree + (1 << j)] += leading * self.vanishing[s][j];
            }
        }
        out.truncate(degree);
        out
    }
    fn recurse(&self, coefficients: &[Ghash], s: usize, offset: Ghash, out: &mut [Ghash]) {
        if s == 0 {
            out[0] = coefficients
                .iter()
                .rev()
                .fold(Ghash::ZERO, |v, &c| v * offset + c);
            return;
        }
        let half = out.len() / 2;
        let right_offset = offset + Ghash(1u128 << (s - 1));
        let left = self.remainder(coefficients, s - 1, offset);
        let right = self.remainder(coefficients, s - 1, right_offset);
        self.recurse(&left, s - 1, offset, &mut out[..half]);
        self.recurse(&right, s - 1, right_offset, &mut out[half..]);
    }
    pub(crate) fn evaluate(&self, coefficients: &[Ghash], size: usize) -> Vec<Ghash> {
        assert!(
            size.is_power_of_two() && size <= 1 << MAX_LOG_DOMAIN && coefficients.len() <= size
        );
        let mut out = vec![Ghash::ZERO; size];
        self.recurse(coefficients, size.ilog2() as usize, Ghash::ZERO, &mut out);
        out
    }
}
fn linearized(coefficients: &[Ghash], mut x: Ghash) -> Ghash {
    coefficients.iter().fold(Ghash::ZERO, |sum, &c| {
        let v = sum + c * x;
        x = x.square();
        v
    })
}
impl ReedSolomon<Ghash> for AdditiveRs {
    fn next_order(&self, size: usize) -> Option<usize> {
        size.max(1)
            .checked_next_power_of_two()
            .filter(|&x| x <= 1 << MAX_LOG_DOMAIN)
    }
    fn generator(&self, _: usize) -> Ghash {
        // Used by upstream multiplicative-domain ZK code, never standard WHIR.
        panic!("an additive RS domain has no multiplicative generator")
    }
    fn evaluation_points(
        &self,
        message_len: usize,
        code_len: usize,
        indices: &[usize],
    ) -> Vec<Ghash> {
        assert!(
            code_len.is_power_of_two()
                && code_len <= 1 << MAX_LOG_DOMAIN
                && message_len <= code_len
        );
        indices
            .iter()
            .map(|&i| {
                assert!(i < code_len);
                Ghash(i as u128)
            })
            .collect()
    }
    fn interleaved_encode(
        &self,
        messages: Messages<'_, Ghash>,
        masks: &Buffer<Ghash>,
        code_len: usize,
    ) -> Buffer<Ghash> {
        assert!(masks.is_empty(), "the binary WHIR profile is non-ZK");
        let width = messages.vectors().len() * messages.interleaving_depth();
        let mut result = vec![Ghash::ZERO; code_len * width];
        for (v, vector) in messages.vectors().iter().enumerate() {
            for (j, message) in vector
                .to_slice()
                .chunks_exact(messages.message_length())
                .enumerate()
            {
                let evaluations = self.evaluate(message, code_len);
                for (row, value) in evaluations.into_iter().enumerate() {
                    result[row * width + v * messages.interleaving_depth() + j] = value;
                }
            }
        }
        Buffer::from(result)
    }
}
pub(crate) fn register() -> &'static Arc<AdditiveRs> {
    static ENCODER: OnceLock<Arc<AdditiveRs>> = OnceLock::new();
    ENCODER.get_or_init(|| {
        let encoder = Arc::new(AdditiveRs::new());
        NTT.insert(encoder.clone() as Arc<dyn ReedSolomon<Ghash>>);
        encoder
    })
}
