use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5a6db5f38dc2d9a9")]
pub struct ClosePositionLogV2Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub price: u64,
    pub price_exponent: i32,
    pub size_amount: u64,
    pub size_usd: u64,
    pub profit_usd: u64,
    pub loss_usd: u64,
    pub fee_amount: u64,
}
