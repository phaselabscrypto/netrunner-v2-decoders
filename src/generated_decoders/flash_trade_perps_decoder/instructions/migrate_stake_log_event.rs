use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3662efd2365a13a8")]
pub struct MigrateStakeLogEvent {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub compounding_amount_out: u64,
    pub pool_lp_amount: u64,
    pub reward_per_lp_staked: u64,
    pub lp_price_usd: u64,
    pub compounding_price_usd: u64,
}
