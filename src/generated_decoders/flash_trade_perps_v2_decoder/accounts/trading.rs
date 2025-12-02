use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x511ea3ae8cf2ef68")]
pub struct Trading {
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub is_initialized: bool,
    pub level: u8,
    pub bump: u8,
    pub voltage_points: u64,
    pub stats: VoltageStats,
    pub snapshot: VoltageStats,
    pub timestamp: i64,
    pub counter: u64,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub burnt: bool,
    pub padding: [u8; 15],
}
