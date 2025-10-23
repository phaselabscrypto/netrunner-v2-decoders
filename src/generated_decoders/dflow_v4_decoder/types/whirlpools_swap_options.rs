use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WhirlpoolsSwapOptions {
    pub amount: u64,
    pub a_to_b: bool,
    pub orchestrator_flags: OrchestratorFlags,
}
