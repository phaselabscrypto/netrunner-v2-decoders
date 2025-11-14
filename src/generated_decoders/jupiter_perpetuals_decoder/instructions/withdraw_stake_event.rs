use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2f55efd6cf1d9758")]
pub struct WithdrawStakeEvent {
    pub custody: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_info: solana_sdk::pubkey::Pubkey,
    pub stake_rewards: u64,
    pub custody_total_staked_amount: u64,
    pub total_staking_rewards: u64,
    pub withdraw_amount: u64,
    pub withdraw_time: i64,
}
