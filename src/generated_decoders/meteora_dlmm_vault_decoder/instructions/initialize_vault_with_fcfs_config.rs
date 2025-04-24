use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbdfb5c68eb1551b6")]
pub struct InitializeVaultWithFcfsConfig {
    pub params: InitializeVaultWithConfigParams,
}

pub struct InitializeVaultWithFcfsConfigInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeVaultWithFcfsConfig {
    type ArrangedAccounts = InitializeVaultWithFcfsConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let quote_mint = accounts.get(2)?;
        let funder = accounts.get(3)?;
        let config = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let event_authority = accounts.get(6)?;
        let program = accounts.get(7)?;

        Some(InitializeVaultWithFcfsConfigInstructionAccounts {
            vault: vault.pubkey,
            pool: pool.pubkey,
            quote_mint: quote_mint.pubkey,
            funder: funder.pubkey,
            config: config.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
