

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2f55efd6cf1d9758")]
pub struct WithdrawStakeEvent{
    pub config: solana_sdk::pubkey::Pubkey,
    pub operator_authority: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub withdraw_to: solana_sdk::pubkey::Pubkey,
    pub settlement_staker_authority: solana_sdk::pubkey::Pubkey,
    pub withdrawn_amount: u64,
}
