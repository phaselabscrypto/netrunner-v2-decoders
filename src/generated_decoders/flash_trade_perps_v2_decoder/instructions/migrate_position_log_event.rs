use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d0aac23f5305a83ce")]
pub struct MigratePositionLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub entry_price: u64,
    pub entry_price_exponent: i32,
    pub size_amount: u64,
    pub size_usd: u64,
    pub collateral_amount: u64,
    pub collateral_usd: u64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
}
