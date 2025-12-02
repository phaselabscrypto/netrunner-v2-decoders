use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1c7b65009dafbdeb")]
pub struct CollectStakeRewardLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool_name: String,
    pub reward_amount: u64,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
}
