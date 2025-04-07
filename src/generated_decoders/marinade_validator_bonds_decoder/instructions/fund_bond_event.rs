

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9c3f9cfc6db5496e")]
pub struct FundBondEvent{
    pub bond: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_authority_signer: solana_sdk::pubkey::Pubkey,
    pub deposited_amount: u64,
}
