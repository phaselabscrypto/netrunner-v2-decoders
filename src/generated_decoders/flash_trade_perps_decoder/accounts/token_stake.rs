use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe57b15f3f6a439ef")]
pub struct TokenStake {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub is_initialized: bool,
    pub bump: u8,
    pub level: u8,
    pub withdraw_request_count: u8,
    pub withdraw_request: [WithdrawRequest; 5],
    pub active_stake_amount: u64,
    pub update_timestamp: i64,
    pub trade_timestamp: i64,
    pub trade_counter: u32,
    pub last_reward_epoch_count: u32,
    pub reward_tokens: u64,
    pub unclaimed_revenue_amount: u64,
    pub revenue_snapshot: u128,
    pub padding: [u64; 1],
}
