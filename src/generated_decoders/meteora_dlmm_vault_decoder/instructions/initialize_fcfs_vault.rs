use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa3cd4591eb472f15")]
pub struct InitializeFcfsVault {
    pub params: InitializeFcfsVaultParams,
}

pub struct InitializeFcfsVaultInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub base: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeFcfsVault {
    type ArrangedAccounts = InitializeFcfsVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let funder = accounts.get(2)?;
        let base = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let event_authority = accounts.get(5)?;
        let program = accounts.get(6)?;

        Some(InitializeFcfsVaultInstructionAccounts {
            vault: vault.pubkey,
            pool: pool.pubkey,
            funder: funder.pubkey,
            base: base.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
