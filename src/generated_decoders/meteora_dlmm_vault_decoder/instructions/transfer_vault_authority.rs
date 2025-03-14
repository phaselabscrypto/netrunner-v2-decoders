

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8b23535834baa26e")]
pub struct TransferVaultAuthority{
    pub new_authority: solana_sdk::pubkey::Pubkey,
}

pub struct TransferVaultAuthorityInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub vault_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferVaultAuthority {
    type ArrangedAccounts = TransferVaultAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let vault_authority = accounts.get(1)?;

        Some(TransferVaultAuthorityInstructionAccounts {
            vault: vault.pubkey,
            vault_authority: vault_authority.pubkey,
        })
    }
}