use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x423e44466cb3b7eb")]
pub struct StakeInfo {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub current_staked_amount_lamports: u64,
    pub total_staking_rewards_lamports: u64,
    pub last_updated_at: i64,
    pub deactivating: bool,
    pub stake_account_index: u64,
    pub bump: u8,
}
