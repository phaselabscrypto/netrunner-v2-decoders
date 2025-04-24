use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateFcfsVaultParams {
    pub max_depositing_cap: u64,
    pub depositing_point: u64,
    pub individual_depositing_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
}
