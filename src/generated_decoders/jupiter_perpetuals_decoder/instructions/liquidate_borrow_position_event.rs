use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d0b80fc3b31c038aa")]
pub struct LiquidateBorrowPositionEvent {
    pub position_key: solana_sdk::pubkey::Pubkey,
    pub position_custody: solana_sdk::pubkey::Pubkey,
    pub position_size_usd: u64,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub collateral_locked_in_usd: u64,
    pub collateral_locked_in_lp: u64,
    pub remaining_collateral_in_lp: u64,
    pub custody_token_price: u64,
    pub total_borrows_in_usd: u64,
    pub liquidation_fee_usd: u64,
    pub liquidation_time: i64,
}
