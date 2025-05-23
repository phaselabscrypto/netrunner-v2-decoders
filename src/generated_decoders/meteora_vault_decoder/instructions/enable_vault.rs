use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9152f19c1a9ae9d3")]
pub struct EnableVault {
    pub enabled: u8,
}

pub struct EnableVaultInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EnableVault {
    type ArrangedAccounts = EnableVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let admin = accounts.get(1)?;

        Some(EnableVaultInstructionAccounts {
            vault: vault.pubkey,
            admin: admin.pubkey,
        })
    }
}
