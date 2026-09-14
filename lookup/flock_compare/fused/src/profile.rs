use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Timings {
    pub source_checks_ms: f64,
    pub decode_ms: f64,
    pub fused_leaf_build_ms: f64,
    pub compact_build_ms: f64,
    pub coefficient_eval_ms: f64,
    pub extension_build_ms: f64,
    pub layer_materialize_ms: f64,
    pub layer_release_ms: f64,
    pub scratch_setup_ms: f64,
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
    Released,
    Fused,
}
