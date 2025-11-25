use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d00f1548d8baada6e")]
pub struct RedeemStakeEvent {
    pub custody: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_info: solana_sdk::pubkey::Pubkey,
    pub stake_rewards: u64,
    pub custody_total_staked_amount: u64,
    pub current_staked_amount: u64,
    pub total_staking_rewards: u64,
    pub redeem_time: i64,
}
