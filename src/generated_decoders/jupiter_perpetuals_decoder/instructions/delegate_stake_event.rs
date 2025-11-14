use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d55874bdea8859fd4")]
pub struct DelegateStakeEvent {
    pub custody: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_info: solana_sdk::pubkey::Pubkey,
    pub custody_total_staked_amount: u64,
    pub stake_amount: u64,
    pub validator_vote_account: solana_sdk::pubkey::Pubkey,
    pub delegate_time: i64,
}
