use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc9d2906cebd1553a")]
pub struct ClaimWithdrawRequestEvent {
    pub withdraw_request: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub split_stake: Option<SplitStakeData>,
    pub new_stake_account_owner: solana_sdk::pubkey::Pubkey,
    pub withdrawing_amount: u64,
    pub withdrawn_amount: U64ValueChange,
}
