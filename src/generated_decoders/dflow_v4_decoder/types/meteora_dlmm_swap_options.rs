use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MeteoraDlmmSwapOptions {
    pub amount: u64,
    pub num_bin_arrays: u8,
    pub orchestrator_flags: OrchestratorFlags,
}
