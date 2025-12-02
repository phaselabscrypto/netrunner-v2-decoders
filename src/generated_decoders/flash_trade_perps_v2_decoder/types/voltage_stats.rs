use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct VoltageStats {
    pub volume_usd: u128,
    pub lp_rewards_usd: u128,
    pub referral_rebate_usd: u128,
}
