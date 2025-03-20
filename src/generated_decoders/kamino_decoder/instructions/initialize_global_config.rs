

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x71d87a83e1d11637")]
pub struct InitializeGlobalConfig{
}

pub struct InitializeGlobalConfigInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeGlobalConfig {
    type ArrangedAccounts = InitializeGlobalConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let global_config = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(InitializeGlobalConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}