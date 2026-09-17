//! Optional timings around the unmodified native prover interface.
use super::F;
use binius_core::protocols::sumcheck::{
    Error, RoundCoeffs, prove::SumcheckProver,
};
use binius_math::EvaluationOrder;
use serde_json::{Value, json};
use std::time::Instant;

#[derive(Default)]
pub struct NativeProfile {
    pub input_adaptation_ms: f64,
    pub builder_ms: f64,
    pub sumcheck_ms: f64,
    execute_ms: Vec<f64>,
    fold_ms: Vec<f64>,
    finish_ms: f64,
}

impl NativeProfile {
    pub fn report(&self) -> Value {
        json!({
            "input_adaptation_ms": self.input_adaptation_ms,
            "builder_including_eq_expansion_ms": self.builder_ms,
            "batch_sumcheck_ms": self.sumcheck_ms,
            "execute_ms_by_round": self.execute_ms,
            "fold_ms_by_round": self.fold_ms,
            "finish_ms": self.finish_ms,
            "scope": "Separate diagnostic run. Timings wrap upstream builder, execute, fold and finish without modifying upstream kernels. Fold includes native equality-table folding.",
        })
    }
}

pub struct Profiled<'a, T> {
    pub inner: T,
    pub timings: &'a mut NativeProfile,
}

impl<T: SumcheckProver<F>> SumcheckProver<F> for Profiled<'_, T> {
    fn n_vars(&self) -> usize { self.inner.n_vars() }
    fn evaluation_order(&self) -> EvaluationOrder { self.inner.evaluation_order() }
    fn execute(&mut self, coefficient: F) -> Result<RoundCoeffs<F>, Error> {
        let start = Instant::now();
        let result = self.inner.execute(coefficient);
        self.timings.execute_ms.push(start.elapsed().as_secs_f64() * 1000.0);
        result
    }
    fn fold(&mut self, challenge: F) -> Result<(), Error> {
        let start = Instant::now();
        let result = self.inner.fold(challenge);
        self.timings.fold_ms.push(start.elapsed().as_secs_f64() * 1000.0);
        result
    }
    fn finish(self: Box<Self>) -> Result<Vec<F>, Error> {
        let start = Instant::now();
        let result = Box::new(self.inner).finish();
        self.timings.finish_ms = start.elapsed().as_secs_f64() * 1000.0;
        result
    }
}
