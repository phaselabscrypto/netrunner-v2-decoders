use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x790754fe97e42b90")]
pub struct TokenVault {
    pub is_initialized: bool,
    pub bump: u8,
    pub token_account_bump: u8,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub token_permissions: TokenPermissions,
    pub withdraw_time_limit: i64,
    pub withdraw_instant_fee: u64,
    pub withdraw_instant_fee_earned: u64,
    pub stake_level: [u64; 6],
    pub tokens_staked: StakeStats,
    pub reward_tokens_to_distribute: u128,
    pub reward_tokens_paid: u128,
    pub tokens_to_distribute: u128,
    pub tokens_distributed: u128,
    pub last_reward_epoch_count: u32,
    pub reward_tokens_distributed: u128,
    pub padding: [u32; 3],
    pub revenue_token_account_bump: u8,
    pub revenue_per_faf_staked: u64,
    pub revenue_accrued: u128,
    pub revenue_distributed: u128,
    pub revenue_paid: u128,
    pub padding2: [u64; 4],
}
