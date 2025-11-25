use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d17798344a8460e4c")]
pub struct BorrowFromCustodyEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position_key: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_custody: solana_sdk::pubkey::Pubkey,
    pub size_custody_token: u64,
    pub collateral_amount: u64,
    pub collateral_amount_usd: u64,
    pub margin_usd: u64,
    pub update_time: i64,
}
