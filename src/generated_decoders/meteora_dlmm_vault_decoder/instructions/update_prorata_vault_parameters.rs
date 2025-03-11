
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb1279732fdf9054a")]
pub struct UpdateProrataVaultParameters{
    pub params: UpdateProrataVaultParams,
}

pub struct UpdateProrataVaultParametersInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateProrataVaultParameters {
    type ArrangedAccounts = UpdateProrataVaultParametersInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let admin = accounts.get(2)?;
        let event_authority = accounts.get(3)?;
        let program = accounts.get(4)?;

        Some(UpdateProrataVaultParametersInstructionAccounts {
            vault: vault.pubkey,
            pool: pool.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}