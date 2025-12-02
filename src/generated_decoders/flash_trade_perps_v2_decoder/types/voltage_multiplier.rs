use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct VoltageMultiplier {
    pub volume: u64,
    pub rewards: u64,
    pub rebates: u64,
}
