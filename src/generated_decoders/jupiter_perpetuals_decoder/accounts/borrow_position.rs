use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xf38c148b20f37237")]
pub struct BorrowPosition {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub borrow_size: u128,
    pub cumulative_compounded_interest_snapshot: u128,
    pub locked_collateral: u64,
    pub bump: u8,
    pub last_borrowed: i64,
}
