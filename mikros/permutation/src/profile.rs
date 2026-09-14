use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Timings {
    pub source_checks_ms: f64,
    /// Online conversion of tree-leaf queries into committed witness claims.
    pub source_reduction_ms: f64,
    /// Revised packed protocol: coefficient aggregation and quotient creation.
    /// Also included in messages_ms; do not sum these attribution fields again.
    #[serde(default)]
    pub packed_aggregate_ms: f64,
    /// Revised packed protocol: online rho/z tables and child contraction.
    /// Also included in layer_materialize_ms.
    #[serde(default)]
    pub packed_projection_ms: f64,
    /// Revised packed protocol: whole extension-tree GKR traversal, overlapping
    /// its equality/message/folding/scratch/release timings below.
    #[serde(default)]
    pub packed_extension_gkr_ms: f64,
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
    Registers,
    Recycled,
    Combined,
}

impl Engine {
    pub(crate) fn fused_tree(self) -> bool {
        matches!(
            self,
            Self::Fused | Self::Registers | Self::Recycled | Self::Combined
        )
    }

    pub(crate) fn tree_pool(self) -> Option<usize> {
        match self {
            Self::Recycled => Some(0),
            Self::Combined => Some(1),
            _ => None,
        }
    }

    pub(crate) fn register_fold(self) -> bool {
        matches!(self, Self::Registers | Self::Combined)
    }
}
