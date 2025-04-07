

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1dff31dbc7770ac3b1")]
pub struct ResetStakeEvent{
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub settlement_staker_authority: solana_sdk::pubkey::Pubkey,
}
