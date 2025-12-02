use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xafb2ab1ebbfd0d76")]
pub struct FlpStake {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub stake_stats: StakeStats,
    pub reward_snapshot: u128,
    pub unclaimed_rewards: u64,
    pub fee_share_bps: u64,
    pub is_initialized: u8,
    pub bump: u8,
}
