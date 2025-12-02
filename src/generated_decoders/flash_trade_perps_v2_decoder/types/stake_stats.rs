use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StakeStats {
    pub pending_activation: u64,
    pub active_amount: u64,
    pub pending_deactivation: u64,
    pub deactivated_amount: u64,
}
