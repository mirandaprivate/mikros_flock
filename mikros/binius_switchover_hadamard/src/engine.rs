use super::{F, Hadamard};
use binius_fast_compute::arith_circuit::ArithCircuitPoly;
use binius_field::PackedField;
use binius_math::{ArithCircuit, CompositionPoly, EvaluationOrder, RowsBatchRef};

#[derive(Clone, Copy)]
pub struct Options {
    pub order: EvaluationOrder,
    pub native_composition: bool,
    pub first_round_zero: bool,
    pub profile: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { order: if super::BYTE_SLICED { EvaluationOrder::HighToLow } else { EvaluationOrder::LowToHigh }, native_composition: true,
            first_round_zero: true, profile: false }
    }
}

#[derive(Debug, Clone)]
pub enum Composition {
    Scalar(Hadamard),
    Native(ArithCircuitPoly<F>),
}

impl Composition {
    pub fn new(native: bool) -> Self {
        if native {
            Self::Native(ArithCircuitPoly::new(
                ArithCircuit::var(0) * ArithCircuit::var(1) + ArithCircuit::var(2)))
        } else { Self::Scalar(Hadamard) }
    }
}

impl<P: PackedField<Scalar = F>> CompositionPoly<P> for Composition {
    fn n_vars(&self) -> usize { 3 }
    fn degree(&self) -> usize { 2 }
    fn binary_tower_level(&self) -> usize { 0 }
    fn expression(&self) -> ArithCircuit<F> {
        ArithCircuit::var(0) * ArithCircuit::var(1) + ArithCircuit::var(2)
    }
    fn evaluate(&self, query: &[P]) -> Result<P, binius_math::Error> {
        match self {
            Self::Scalar(poly) => poly.evaluate(query),
            Self::Native(poly) => poly.evaluate(query),
        }
    }
    fn batch_evaluate(&self, query: &RowsBatchRef<P>, output: &mut [P]) -> Result<(), binius_math::Error> {
        match self {
            Self::Scalar(poly) => poly.batch_evaluate(query, output),
            Self::Native(poly) => poly.batch_evaluate(query, output),
        }
    }
}
