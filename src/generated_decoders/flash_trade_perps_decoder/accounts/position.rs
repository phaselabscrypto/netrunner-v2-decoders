use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xaabc8fe47a40f7d0")]
pub struct Position {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub entry_price: OraclePrice,
    pub size_amount: u64,
    pub size_usd: u64,
    pub locked_amount: u64,
    pub locked_usd: u64,
    pub collateral_amount: u64,
    pub collateral_usd: u64,
    pub unsettled_amount: u64,
    pub unsettled_fees_usd: u64,
    pub cumulative_lock_fee_snapshot: u128,
    pub degen_size_usd: u64,
    pub buffer: u128,
    pub size_decimals: u8,
    pub locked_decimals: u8,
    pub collateral_decimals: u8,
    pub bump: u8,
    pub padding: [u8; 12],
}
