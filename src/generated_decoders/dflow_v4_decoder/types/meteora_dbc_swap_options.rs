use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MeteoraDbcSwapOptions {
    pub amount: u64,
    pub is_rate_limiter_applied: bool,
    pub orchestrator_flags: OrchestratorFlags,
}
