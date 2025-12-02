use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d49af08c87cb98bee")]
pub struct CollectRevenueLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_stake: solana_sdk::pubkey::Pubkey,
    pub revenue_amount: u64,
    pub last_epoch_count: u32,
    pub padding: [u64; 2],
}
