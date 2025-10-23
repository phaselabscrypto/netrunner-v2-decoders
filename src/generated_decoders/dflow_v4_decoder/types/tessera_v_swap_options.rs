use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TesseraVSwapOptions {
    pub amount: u64,
    pub orchestrator_flags: OrchestratorFlags,
}
