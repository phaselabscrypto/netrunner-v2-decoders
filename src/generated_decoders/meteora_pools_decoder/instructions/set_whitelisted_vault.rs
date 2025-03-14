

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0c945e2a373953f7")]
pub struct SetWhitelistedVault{
    pub whitelisted_vault: solana_sdk::pubkey::Pubkey,
}

pub struct SetWhitelistedVaultInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetWhitelistedVault {
    type ArrangedAccounts = SetWhitelistedVaultInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let admin = accounts.get(1)?;

        Some(SetWhitelistedVaultInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}