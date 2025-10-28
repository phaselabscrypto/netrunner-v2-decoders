use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AirdropBonus {
    pub total_bonus: u64,
    pub vesting_duration: u64,
    pub total_claimed_bonus: u64,
}
