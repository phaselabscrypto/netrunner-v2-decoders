use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x6a2fee9f7c0ca0c0")]
pub struct LockConfig {
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_owner: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub locked_timestamp: u64,
    pub lock_type: LockTypeLabel,
}
