use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x4af60671f9e44ba9")]
pub struct Locker {
    pub base: solana_sdk::pubkey::Pubkey,
    pub bump: u8,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub locked_supply: u64,
    pub total_escrow: u64,
    pub governor: solana_sdk::pubkey::Pubkey,
    pub params: LockerParams,
    pub buffers: [u128; 32],
}
