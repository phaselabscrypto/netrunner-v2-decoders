

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d7d7fa056f76e32ee")]
pub struct InitConfigEvent{
    pub config: solana_sdk::pubkey::Pubkey,
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub operator_authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_lockup_epochs: u64,
    pub epochs_to_claim_settlement: u64,
    pub minimum_stake_lamports: u64,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub slots_to_start_settlement_claiming: u64,
}
