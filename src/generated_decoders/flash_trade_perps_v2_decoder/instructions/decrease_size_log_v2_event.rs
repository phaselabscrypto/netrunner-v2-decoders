use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d39193d235e979713")]
pub struct DecreaseSizeLogV2Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub price: u64,
    pub price_exponent: i32,
    pub size_delta: u64,
    pub size_delta_usd: u64,
    pub settled_returns: u64,
    pub delta_profit_usd: u64,
    pub delta_loss_usd: u64,
    pub fee_amount: u64,
}
