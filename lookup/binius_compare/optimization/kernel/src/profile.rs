use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Timings {
    pub packed_quotient_ms: f64,
    pub packed_projection_ms: f64,
    pub packed_columns_ms: f64,
    pub packed_layers: usize,
    pub skipped_rounds: usize,
    pub source_checks_ms: f64,
    pub decode_ms: f64,
    pub compact_build_ms: f64,
    pub coefficient_eval_ms: f64,
    pub extension_build_ms: f64,
    pub layer_materialize_ms: f64,
    pub equality_ms: f64,
    pub messages_ms: f64,
    pub folds_ms: f64,
    pub fold_and_message_ms: f64,
}
pub(crate) fn elapsed(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Engine {
    Reference,
    ParallelTree,
    LookupTree,
    Fused,
}
