use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd0c1a6389c099286")]
pub struct CompoundingFeesLogEvent {
    pub pool_name: String,
    pub reward_amount: u64,
    pub reward_lp_amount: u64,
    pub reward_per_lp_staked: u64,
    pub lp_price_usd: u64,
    pub compounding_price_usd: u64,
}
