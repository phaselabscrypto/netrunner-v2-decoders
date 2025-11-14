use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d71650b618a2e71d3")]
pub struct ClosePositionLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub price_usd: u64,
    pub size_amount: u64,
    pub size_usd: u64,
    pub profit_usd: u64,
    pub loss_usd: u64,
    pub fee_amount: u64,
}
