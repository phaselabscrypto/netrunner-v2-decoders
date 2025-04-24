use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xac170d8f128568ae")]
pub struct UpdateFcfsVaultParameters {
    pub params: UpdateFcfsVaultParams,
}

pub struct UpdateFcfsVaultParametersInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFcfsVaultParameters {
    type ArrangedAccounts = UpdateFcfsVaultParametersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let admin = accounts.get(2)?;
        let event_authority = accounts.get(3)?;
        let program = accounts.get(4)?;

        Some(UpdateFcfsVaultParametersInstructionAccounts {
            vault: vault.pubkey,
            pool: pool.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
