use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FeesStats {
    pub accrued: u128,
    pub distributed: u128,
    pub paid: u128,
    pub reward_per_lp_staked: u64,
    pub protocol_fee: u64,
}
