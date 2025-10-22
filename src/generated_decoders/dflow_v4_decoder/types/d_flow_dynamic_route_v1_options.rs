use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DFlowDynamicRouteV1Options {
    pub candidate_actions: Vec<DynamicRouteV1CandidateAction>,
    pub amount: u64,
    pub orchestrator_flags: OrchestratorFlags,
}
