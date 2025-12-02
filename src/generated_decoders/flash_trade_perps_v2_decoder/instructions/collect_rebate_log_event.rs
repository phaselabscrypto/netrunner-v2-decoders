use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d0f87db408380742e")]
pub struct CollectRebateLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_stake: solana_sdk::pubkey::Pubkey,
    pub rebate_amount: u64,
    pub rebate_usd: u64,
    pub padding: [u64; 2],
}
